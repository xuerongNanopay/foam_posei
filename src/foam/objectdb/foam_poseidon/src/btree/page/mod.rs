#![allow(unused)]

use std::{mem::ManuallyDrop, ptr, sync::atomic::{AtomicPtr, AtomicUsize, Ordering}};

use crate::{error::{FP_ILLEGAL_ARGUMENT, FP_NO_SUPPORT}, internal::{FPResult, FPTimeStamp, FPTxnId}, util::ptr::layout_ptr::LayoutPtr, FP_ALLOC, FP_BIT_REVERSE_32, FP_BIT_REVERSE_64, FP_REINTERPRET_CAST_BUF, FP_SIZE_OF};

use super::{btree::BTree, row::{RowKeyMem, RowLeaf}, tuple::{self, Tuple, TupleHeader, TupleType}, zone_map::ZMPage, FP_BTREE_PAGE_ADDR_MAX_LENGTH};


#[repr(C)]
#[derive(Clone, Copy)]
pub(crate) union PageHeaderV {
    entries: u32,
    datalen: u32,
}

type PageFlag = u8;

pub(crate) const FP_BTREE_PAGE_COMPRESSED: PageFlag = 0x01;
pub(crate) const FP_BTREE_PAGE_ENCRYPTED:  PageFlag = 0x01 << 1;
pub(crate) const FP_BTREE_PAGE_UNUSED:     PageFlag = 0x01 << 2;

/**
 * Page Header.
 */
#[repr(C)]
#[derive(Clone, Copy)]
pub(crate) struct PageHeader {
    pub(crate) record_number: u64, /* column-store */
    pub(crate) write_epoch: u64,
    pub(crate) mem_size: u32,
    pub(crate) v: PageHeaderV,
    pub(crate) r#type: u8,

    pub(crate) flags: PageFlag,
    pub(crate) unused: u8,
    pub(crate) version: u8,
}

impl PageHeader {
    fn endian_swap(&mut self) {
        if cfg!(target_endian = "big") { 
            self.record_number = FP_BIT_REVERSE_64!(self.record_number);
            self.write_epoch = FP_BIT_REVERSE_64!(self.write_epoch);
            self.mem_size = FP_BIT_REVERSE_32!(self.mem_size);
            self.v.entries = unsafe {
                FP_BIT_REVERSE_32!(self.v.entries)
            };
        }
    }
}

impl From<&[u8]> for PageHeader {
    fn from(raw_data: &[u8]) -> Self {
        let raw_header = FP_REINTERPRET_CAST_BUF!(raw_data, PageHeader);
        let mut page_header = *raw_header;
        page_header.endian_swap();
        page_header
    }
}

/**
 * PageRef type.
 */

 #[repr(usize)]
pub(crate) enum PageRefType {
    Internal = 0,
    Leaf = 1,
}

 #[repr(usize)]
 #[derive(Clone, Copy)]
pub(crate) enum PageRefState {
    Disk = 0,         /* Page is on disk. */
    Deleted = 1,      /* Page is on disk, but deleted */
    Locked = 2,       /* Page locked for exclusive access */
    InMemory = 3,     /* Page is in cache and memory */
    Split = 4,        /* Parent page split */
}

#[repr(usize)]
pub(crate) enum PageReadingState {
    Prefetch = 0, /* Page is in the pre-fetch queue. */
    Reading = 1,  /* Page is reading. */
}

impl TryFrom<usize> for PageReadingState {
    type Error = ();

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(PageReadingState::Prefetch),
            1 => Ok(PageReadingState::Reading),
            _ => Err(()), // Invalid value handling
        }
    }
}


#[repr(C)]
pub(crate) enum PageRefKey {
    Row(*mut (), usize),  /* On-page row key*/
    RowMem(RowKeyMem),    /* In-memory row key */
    Col(u64),             /* column */
}

#[repr(usize)]
#[derive(Clone, Copy)]
pub(crate) enum PageAddrType {
    None = 0,
    Internal = 1,     /* Internal page */
    Leaf = 2,         /* Leaf page */
    LeafNone = 3, /* Leaf page with overflow */
}

#[repr(C)]
pub(crate) struct PageOffAddr {
    pub(crate) zm: ZMPage,
    pub(crate) addr: Vec<u8>,
    pub(crate) r#type: PageAddrType,
}

#[repr(C)]
pub(crate) enum PageRefAddr {
    None,
    Off(PageOffAddr), /* off-page page address */
    In(TupleHeader),  /* in-page page address */
}

#[repr(C)]
#[derive(Clone, Copy)]
pub(crate) struct PageAddr {
    pub(crate) r#type: PageAddrType,
    pub(crate) addr: [u8; FP_BTREE_PAGE_ADDR_MAX_LENGTH],
    pub(crate) size: u8,
    // pub(crate) zm: ZMPage,

    pub(crate) del: Option<PageDel>,
}

/**
 * A wrapper for page, store/keep the metadate for b-tree page.
 */
#[repr(C)]
pub(crate) struct PageRef {
    pub(crate) page: Option<LayoutPtr<Page>>,
    pub(crate) home: Option<&'static Page>,
    pub(crate) unused: u8,
    pub(crate) r#type: PageRefType,
    state: PageRefState,
    read_state: AtomicUsize,
    
    pub(crate) key: PageRefKey,

    pub(crate) addr: PageRefAddr, /* page address info. */
    
    pub(crate) page_header: Option<PageHeader>,
    // page_status: pageStatus, /* prefetch/reading */
}

impl PageRef {
    pub(crate) fn get_ref_key(&self) -> FPResult<(*const (), usize)> {
        match &self.key {
            PageRefKey::Row(p, s) => {
                Ok((*p, *s))
            },
            PageRefKey::RowMem(k) => {
                Ok((k.ptr as *const (), k.len()))
            },
            _ => Err(FP_NO_SUPPORT)
        }
    }

    /**
     * return true if it refers to the root.
     */
    #[inline(always)]
    pub(crate) fn is_root(&self) -> bool {
        match self.home {
            None => true,
            _ => false,
        }
    }

    /**
     * Get status volatile.
     */
    #[inline(always)]
    pub(crate) fn get_state(&self) -> PageRefState {
        let p: *const PageRefState = &self.state as *const PageRefState;
        unsafe { p.read_volatile() }
    }

    /**
     * Get status volatilely.
     */
    #[inline(always)]
    pub(crate) fn set_state(&mut self, state: PageRefState) {
        let p: *mut PageRefState = &mut self.state as *mut PageRefState;
        unsafe { p.write_volatile(state); }
    }

    #[inline(always)]
    pub(crate) fn set_read_state(&mut self, read_state: PageReadingState) {
        self.read_state.store(read_state as usize, Ordering::SeqCst);
    }

    #[inline(always)]
    pub(crate) fn get_read_state(&mut self) -> PageReadingState {
        match PageReadingState::try_from(self.read_state.load(Ordering::SeqCst)) {
            Ok(e) => e,
            Err(_) => panic!("get_read_state")
        }
    }

    /**
     * CAS status
     * return true if set successfully.
     */
    #[inline(always)]
    pub(crate) fn cas_state(&mut self, mut old_state: PageRefState, mut new_state: PageRefState) -> bool {
        let ptr: *mut PageRefState = &mut self.state as *mut PageRefState;
        let a_ptr = AtomicPtr::new(ptr);

        unsafe {
            match a_ptr.compare_exchange(&mut old_state, &mut new_state, Ordering::SeqCst, Ordering::SeqCst) {
                Ok(_) => true,
                Err(_) => false,
            }
        }
    }

    /**
     * Return address of the page.
     */
    pub(crate) fn page_address(&self, btree: &BTree) -> Option<PageAddr> {
        
        let addr = &self.addr;

        match addr {
            PageRefAddr::None => {
                return None;
            },
            PageRefAddr::Off(off_addr) => {
                let mut addr = [0u8; FP_BTREE_PAGE_ADDR_MAX_LENGTH];
                addr.copy_from_slice(&off_addr.addr);
                return Some(PageAddr{
                    r#type: off_addr.r#type,
                    addr,
                    size: off_addr.addr.len() as u8,
                    // zm: off_addr.zm,
                    del: None,
                })
            },
            PageRefAddr::In(in_addr) => {
                let tuple = Tuple::new(in_addr).unwrap();
                let tuple_addr = match tuple {
                    Tuple::Addr(a) => a,
                    _ => panic!("page_address impossible"),
                };
                let mut addr = [0u8; FP_BTREE_PAGE_ADDR_MAX_LENGTH];
                addr.copy_from_slice(&tuple_addr.common.data);

                return Some(PageAddr{
                    r#type: match tuple_addr.common.raw_type {
                        TupleType::AddrInternal => {
                            PageAddrType::Internal
                        },
                        TupleType::AddrLeaf => {
                            PageAddrType::Leaf
                        },
                        TupleType::AddrLeafNone => {
                            PageAddrType::LeafNone
                        },
                        TupleType::AddrDel => {
                            PageAddrType::LeafNone
                        },
                        _ => panic!("page_address impossible"),
                    },
                    addr,
                    size: tuple_addr.common.data.len() as u8,
                    // txn: off_addr.zm,
                    del: if matches!(tuple_addr.common.raw_type, TupleType::AddrDel) {
                        tuple_addr.delete_marker
                    } else { None },
                })
            },
        }
    }
}

#[repr(C)]
pub(crate) union PageContent {
    pub(crate) internal: ManuallyDrop<BtreeInternal>,
    /* no need to clean it */
    pub(crate) row_leaf: *mut RowLeaf,
}

/**
 * Btree Internal page(both row and cloumn store).
 */
#[repr(C)]
pub(crate) struct BtreeInternal {
    pub(crate) parent: *const PageRef,
    pub(crate) split_generation: u64,
    pub(crate) page_index: LayoutPtr<PageIndex>,
}

/**
 * The page index held by each BtreeInternal page.
 */
#[repr(C)]
pub(crate) struct PageIndex {
    pub(crate) entries: u32,
    pub(crate) deleted_entries: u32,
    // pub(crate) indexes: *mut LayoutPtr<PageRef>,
    pub(crate) indexes: *mut LayoutPtr<PageRef>,
}

impl Drop for PageIndex {
    fn drop(&mut self) {
        unsafe {
            for i in 0..self.entries {
                let mut p = self.indexes.add(i as usize);
                if !p.is_null() {
                    ptr::drop_in_place(p);
                    p = ptr::null_mut();
                }

            }
        }
    }
}


#[repr(C)]
pub(crate) enum PageType {
    ColumnFix,
    ColumnVar,
    ColumnIntl,
    Internal,
    RowLeaf,
    Overflow,
}

#[repr(C)]
pub(crate) struct Page {
    pub(crate) r#type: PageType,
    // read_gen: EvictRule,
    pub(crate) entries: usize, /* Leaf page entries */
    pub(crate) content: PageContent,
    // row_leaf_page: BtreePageRow,
    // col_fix_leaf_page: BtreePageColFix,
    // col_var_leaf_page: BtreePageColVar,

    pub(crate) page_header: Option<&'static PageHeader>,


    // leaf_entries: u32,
}

impl Drop for Page {
    fn drop(&mut self) {
        if let PageType::Internal = self.r#type {
            unsafe {
                ManuallyDrop::drop(&mut self.content.internal);
            }
        }
        if let PageType::RowLeaf = self.r#type {
            unsafe {
                for i in 0..self.entries {
                    let mut p = self.content.row_leaf.add(i);
                    if !p.is_null() {
                        ptr::drop_in_place(p);
                        p = ptr::null_mut();
                    }
    
                }
            }
        }
    }
}

impl Page {

    /**
     * Create or read a page.
     */
    pub(crate) fn new(
        page_type: PageType,
        alloc_entries: usize,
        is_alloc_indexes: bool,
    ) -> FPResult<(LayoutPtr<Page>)> {

        let mut mem_size: usize = 0;
    
        let tree_page: LayoutPtr<Page> = match page_type {
            // Create tree page for row BtreeInternal page.
            PageType::Internal => {
                // Create tree page.
                let (l1, p_tree_page) = FP_ALLOC!{
                    Page: 1,
                }?;
                let mut tree_page = LayoutPtr::new(l1, p_tree_page);
                mem_size += FP_SIZE_OF!(Page);

                // Create index for row store BtreeInternal page.
                let (l2, p_page_index, indexes) = FP_ALLOC!{
                    PageIndex: 1,
                    LayoutPtr<PageRef>: alloc_entries,
                }?;
                mem_size += FP_SIZE_OF!(PageIndex) + alloc_entries * FP_SIZE_OF!(* mut PageRef);
                
                let mut page_index = LayoutPtr::new(l2, p_page_index);

                unsafe {
                    page_index.entries = alloc_entries as u32;
                    page_index.indexes = indexes;
                    tree_page.content = PageContent {
                        internal:  ManuallyDrop::new(BtreeInternal{
                            parent: ptr::null_mut(),
                            split_generation: 0,
                            page_index,
                        })
                    };

                    if is_alloc_indexes {
                        for i in 0..alloc_entries {
                            let c_ptr: *mut LayoutPtr<PageRef> = (*p_page_index).indexes.add(i);
                            let (l, p) = FP_ALLOC!{
                                PageRef: 1,
                            }?;
                            *c_ptr = LayoutPtr::new(l, p);
                            mem_size += FP_SIZE_OF!(PageRef);
                        }
                    }
                }

                tree_page
            },
            // Create tree page for row leaf page.
            PageType::RowLeaf => {
                let (l1, p_tree_page, p_page_row) = FP_ALLOC!{
                    Page: 1,
                    RowLeaf: alloc_entries,
                }?;
                let mut tree_page = LayoutPtr::new(l1, p_tree_page);
                mem_size += FP_SIZE_OF!(Page);

                unsafe {
                    (*tree_page).content = PageContent {
                        row_leaf: p_page_row,
                    };
                    (*tree_page).entries= alloc_entries;
                }

                tree_page
            }
            _ => panic!("not support"),
        };

        Ok(tree_page)
    }

    pub(crate) fn get_page_index(&self) -> FPResult<&LayoutPtr<PageIndex>> {
        if !matches!(self.r#type, PageType::ColumnIntl) || !matches!(self.r#type, PageType::Internal) {
            return Err(FP_ILLEGAL_ARGUMENT);
        }
        Ok(unsafe { &self.content.internal.page_index })
    }

    /**
     * Mark the page dirty.
     */
    pub(crate) fn set_modify(&mut self) -> FPResult<()> {
        Ok(())
    }

    /**
     * Read a page.
     * __page_read
     */
    pub(crate) fn read_page(&self) -> FPResult<()> {

        Err(FP_NO_SUPPORT)
    }
}

#[repr(C)]
pub(crate) struct PageModify {
    pub(crate) first_dirty_txn_id: u64,

    pub(crate) last_eviction_echo: u64,
    pub(crate) last_eviction_id: u64,
    pub(crate) last_eviction_timestamp: u64,
}

#[repr(C)]
#[derive(Default, Clone, Copy)]
pub(crate) struct PageDel {
    pub(crate) txn_id: FPTxnId,
    pub(crate) delete_timestamp: FPTimeStamp,
    pub(crate) delete_committed_timestamp: FPTimeStamp,
    pub(crate) committed: bool,
    pub(crate) is_sync: bool,
}