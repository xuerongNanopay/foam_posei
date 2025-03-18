#![allow(unused)]

use std::sync::atomic::{AtomicBool, Ordering};

use crate::{disk::DiskManager, error::FP_NO_IMPL, internal::FPResult};

pub mod btree_cursor;

pub(crate) struct Btree {
    modified: AtomicBool,
    disk_manager: Box<dyn DiskManager>
}

impl Btree {

    pub(super) fn set_modified(&self, modified: bool) {
        self.modified.store(modified, Ordering::Release);
    }

    pub(super) fn get_modified(&self) -> bool {
        self.modified.load(Ordering::Acquire)
    }

    /**
     * Read page from file system.
     */
    pub(super) fn read_page(&self, addr: &[u8]) -> FPResult<Vec<u8>> {

        Err(FP_NO_IMPL)
    }
}

// use std::{mem::ManuallyDrop, ptr, str::FromStr, sync::{atomic::{AtomicBool, AtomicUsize, Ordering}, Arc, Weak}, task::Context};

// use crate::{cursor::CursorItem, error::{FP_BT_PAGE_READ_NOT_FOUND, FP_BT_PAGE_READ_RETRY, FP_NO_IMPL, FP_NO_SUPPORT}, internal::{FPResult, FPRwLock}, scheme::key::KeyOrd, util::ptr::layout_ptr::LayoutPtr, FP_ALLOC, FP_BIT_IST, FP_SIZE_OF};

// use super::{block::manager::BlockManager, buf::BufferPool, page::{Page, PageReadingState, PageRef, PageRefAddr, PageRefKey, PageRefState, PageRefType, PageType}, row::RowKeyMem, BtreeReadFlag, FP_BTEE_READ_CACHE_ONLY, FP_BTEE_READ_NEED_ONCE, FP_BTEE_READ_NO_SPLIT, FP_BTEE_READ_NO_WAIT, FP_BTEE_READ_OVER_CACHE, FP_BTEE_READ_SKIP_DELETED};


// enum BTreeStoreOriented {
//     ColumnFix,
//     ColumnVar,
//     Row
// }

// impl std::fmt::Display for BTreeStoreOriented {
//     fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
//         match self {
//             BTreeStoreOriented::ColumnFix => write!(f, "ColumnFix"),
//             BTreeStoreOriented::ColumnVar => write!(f, "ColumnVar"),
//             BTreeStoreOriented::Row => write!(f, "Row"),
//         }
//     }
// }

// #[derive(Default)]
// pub(crate) enum BTreeType {
//     ColumnFix,
//     ColumnVar,
//     #[default]
//     Row,
// }


// pub type BtreeFlag = u32;
// pub const FP_BTREE_APPEND:    BtreeFlag = 1 << 0;
// pub const FP_BTREE_BULK:      BtreeFlag = 1 << 12;  /* Bulk-load */
// pub const FP_BTREE_CLOSED:    BtreeFlag = 1 << 13;  /* Closed */
// pub const FP_BTREE_IN_MEMORY: BtreeFlag = 1 << 14;  /* In-Memory */
// pub const FP_BTREE_RECOVER:   BtreeFlag = 1 << 15;  /* Recover */
// pub const FP_BTREE_VERIFY:    BtreeFlag = 1 << 16;  /* Verify */

// #[derive(Default)]
// pub(crate) struct BTreeBuilder {
//     name: String,
//     buffer_pool: Option<Arc<BufferPool>>,
//     btree_type: BTreeType,
//     btree_flag: BtreeFlag,
//     //TODO: blk_handle builder.
// }

// impl BTreeBuilder {
//     pub(crate) fn new(
//         uri: &str,
//     ) -> Self {
//         Self {
//             name: uri.to_owned(),
//             ..Default::default()
//         }
//     }

//     fn set_buffer_pool(&mut self, buffer_pool: Arc<BufferPool>) {
//         self.buffer_pool = Some(buffer_pool);
//     }

//     fn set_btree_type(&mut self, btree_type: BTreeType) {
//         self.btree_type = btree_type;
//     }

//     pub(crate) fn open_or_create(self) -> FPResult<BTree> {
//         Err(FP_NO_IMPL)
//     }

//     fn open(self) -> FPResult<BTree> {
//         Err(FP_NO_IMPL)
//     }

//     fn create(self) -> FPResult<BTree> {
//         let block_manager = BlockManager::new(&self.name).unwrap();
        
//         Err(FP_NO_IMPL)
//     }
// }

// pub(crate) struct BTree {
//     initial: AtomicBool,

//     // store_oriented: BTreeStoreOriented,

//     /* Store oriented: row, var length column, fix length column */
//     pub(crate) r#type: BTreeType,

//     /* Root page reference. */
//     root: PageRef,

//     pub(crate) flags: BtreeFlag,

//     // pub(crate) key_cmp_fn: Option<Box<dyn KeyOrd>>,
//     // pub(crate) lower_bound: CursorItem,
//     // pub(crate) upper_bound: CursorItem,

//     // page_cache: Option<Arc<FPRwLock<u32>>>,
//     block_manager: BlockManager,
//     // k_format: String,
//     // v_format: String,
//     // fixed_length_field_size: u8,

//     // logging_file_id: u32,

//     // allocation_size: u32,
//     // max_internal_size: u32,
//     // max_leaf_page: u32,
//     // max_leaf_key: u32,
//     // max_leaf_value: u32,
//     // max_mem_page: u32,
//     // mem_page_split_throttle: u64,

//     // dictionary: u32,
//     // internal_key_truncate: bool,
//     // prefix_compression: bool,

//     // split_percentage: i32,

//     // block_header: u32,

//     // block_manager: Arc<BlockManager>
// }

// /**
//  * meta data.
//  */
// impl BTree {

//     pub(crate) fn builder(name: &str) -> BTreeBuilder {
//         BTreeBuilder::new(name)
//     }

//     /*
//     * Create or reopen a btree.
//     */
//     fn open(flag: BtreeFlag) -> FPResult<()> {
//         Ok(())
//     }

//     /*
//     * Initial an empty in-memory B-tree.
//     */
//     fn initial_empty(btree: &mut LayoutPtr<BTree>) -> FPResult<()> {
//         match btree.r#type {
//             BTreeType::Row => {
//                 // First b-tree page(Internal).
//                 let mut root_page: LayoutPtr<Page> = Page::new(PageType::Internal, 1, true)?;
//                 unsafe {
//                     // root page parent is root page ref.
//                     (*root_page.content.internal).parent = &btree.root as *const PageRef;

//                     // Initial leaf page ref.
//                     let first_page_ref: *mut LayoutPtr<PageRef> = root_page.content.internal.page_index.indexes;
//                     (*first_page_ref).home = Some(root_page.as_ref());
//                     (*first_page_ref).page = None;
//                     (*first_page_ref).addr = PageRefAddr::None;
//                     (*first_page_ref).r#type = PageRefType::Leaf;
//                     (*first_page_ref).set_state(PageRefState::Deleted);
//                     // Give the a initial key `""`
//                     (*first_page_ref).key = PageRefKey::RowMem(Self::row_mem_key_init("")?);


//                     // Initial first leaf page if bulk load on.
//                     if FP_BIT_IST!(btree.flags, FP_BTREE_BULK) {
//                         Self::new_leaf_page(btree, &mut *first_page_ref)?;
//                         (*first_page_ref).r#type = PageRefType::Leaf;
//                         (*first_page_ref).set_state(PageRefState::InMemory);
//                         //TODO: PageModify.
//                     }

//                 }

//                 Self::root_ref_init(btree, root_page, !matches!(btree.r#type,BTreeType::Row));
//             },
//             _ => return Err(FP_NO_SUPPORT)
//         };
//         Ok(())
//     }

//     /**
//      * Initial Btree root page ref.
//      */
//     fn root_ref_init(btree: &mut LayoutPtr<BTree>, mut root_page: LayoutPtr<Page>, is_col_store: bool) {

//         unsafe {
//             /* root internal page parent point to root ref in btree, another word root parent is root. */
//             (*root_page.content.internal).parent = &btree.root as *const PageRef;   
//         }

//         btree.root.page = Some(root_page);
//         btree.root.r#type = PageRefType::Internal;
//         btree.root.set_state(PageRefState::InMemory);


//         if is_col_store {
//             btree.root.key = PageRefKey::Col(1);
//         }
//     }

//     /**
//      * Create a new leaf page for both row and column store.
//      */
//     pub(crate) fn new_leaf_page(btree: & LayoutPtr<BTree>, page_ref: &mut LayoutPtr<PageRef>) -> FPResult<()> {

//         let page = match btree.r#type {
//             BTreeType::Row => {
//                 Page::new(PageType::RowLeaf, 0, false)
//             },
//             _ => {
//                 return Err(FP_NO_SUPPORT)
//             }
//         }?;
//         page_ref.page = Some(page);
//         page_ref.r#type = PageRefType::Leaf;

//         Ok(())
//     }

//     /**
//      * Initial a b-tree memory key.
//      */
//     fn row_mem_key_init(key: &str) -> FPResult<RowKeyMem>{
//         //TODO: memory metric
//         RowKeyMem::from_str(key)
//     }

//     pub(crate) fn disable_bulk_load(&self) {
//         if !*&self.initial.load(Ordering::Relaxed) {
//             return;
//         }
//         &self.initial.store(false, Ordering::SeqCst);

        
//     }

//     fn read_page(&mut self, ctx: &mut Context, read_ref: &mut PageRef, flags:BtreeReadFlag) -> FPResult<()>  {
//         //1.lock
//         //2.check/classify.

//         /* Lock the PageRef. */
//         let previous_state = read_ref.get_state();
//         match previous_state {
//             PageRefState::Deleted | PageRefState::Disk => {
//                 if !read_ref.cas_state(previous_state, PageRefState::Locked) {
//                     return Ok(())
//                 }
//             },
//             _ => {
//                 return Ok(())
//             },
//         };

//         if matches!(previous_state, PageRefState::Disk) {
//             read_ref.set_read_state(PageReadingState::Reading);
//         };

//         //NEED TODO: consider delete page.
//         if matches!(previous_state, PageRefState::Deleted) {

//         }

//         //TODO: handle non.
//         let page_addr = read_ref.page_address(self).unwrap();

//         //NEED TODO: handle deleted page ref.



//         Err(FP_NO_IMPL)
//     }

//     fn page_blk_addr(&self, read_ref: &mut PageRef) -> bool {

//         let addr = &read_ref.addr;


//         if let PageRefAddr::None = addr {
//             return false;
//         }

        

//         return false;
//     }

//     /**
//      * Read Page.
//      * __wt_page_in_func
//      */
//     fn load_page_ref(&mut self, ctx: &mut Context, read_ref: &mut PageRef, flags:BtreeReadFlag) -> FPResult<()> {
//         let mut evit_skip = false;
//         let mut read_from_disk = false;
//         let mut stalled = false;
//         let mut wont_need = false;
//         let mut current_status: PageRefState;
//         let mut busy = false;
//         let read = || {

//         };

//         'load_page: loop {
//             current_status = read_ref.get_state();
//             match current_status {
//                 PageRefState::Deleted => {
//                     if FP_BIT_IST!(flags, FP_BTEE_READ_CACHE_ONLY | FP_BTEE_READ_NO_WAIT) {
//                         return Err(FP_BT_PAGE_READ_NOT_FOUND);
//                     }
//                     //TODO: Need to consider transaction/snapshot.
//                     if FP_BIT_IST!(flags, FP_BTEE_READ_SKIP_DELETED) {
//                         return Err(FP_BT_PAGE_READ_NOT_FOUND);
//                     }
//                     //TODO: goto read.
//                 },
//                 PageRefState::Disk => {
//                     if FP_BIT_IST!(flags, FP_BTEE_READ_CACHE_ONLY) {
//                         return Err(FP_BT_PAGE_READ_NOT_FOUND);
//                     }
//                     if FP_BIT_IST!(flags, FP_BTEE_READ_OVER_CACHE) {
//                         //TODO: check the used cache size.
//                     }

//                     self.read_page(ctx, read_ref, flags)?;
//                     read_from_disk = true;
//                     evit_skip = true;

//                     if FP_BIT_IST!(flags, FP_BTEE_READ_NEED_ONCE) {
//                         wont_need = true
//                     }
//                     continue;
//                 },
//                 PageRefState::Locked => {
//                     if FP_BIT_IST!(flags, FP_BTEE_READ_NO_WAIT) {
//                         return Err(FP_BT_PAGE_READ_NOT_FOUND);
//                     }

//                     if matches!(read_ref.get_read_state(), PageReadingState::Reading) {
//                         if FP_BIT_IST!(flags, FP_BTEE_READ_CACHE_ONLY) {
//                             return Err(FP_BT_PAGE_READ_NOT_FOUND);
//                         }
//                     }
//                     stalled = true;
//                     break;
//                 },
//                 PageRefState::Split => {
//                     return Err(FP_BT_PAGE_READ_RETRY);
//                 },
//                 PageRefState::InMemory => {
//                     'evict: loop {
//                         if FP_BIT_IST!(self.flags, FP_BTREE_IN_MEMORY) {
//                             break;
//                         }
//                         //MUST TODO: register harzard pointer.
//                         //__wt_hazard_set_func
//                         //TODO: check busy.
//                         if busy {
//                             continue 'load_page;
//                         }

//                         if evit_skip || FP_BIT_IST!(flags, FP_BTEE_READ_NO_SPLIT) {
//                             break;
//                         }

//                         //MUST TODO: trigger evict/page_split if a page is to large.
//                         break;
//                     }

//                     let page = read_ref.page.as_ref().unwrap();

//                     //TODO: prefetch optimization.
//                     //TODO: eviction strategy.
//                     //TODO: transaction consideration.


//                 },
//             };
//         }

//         //TODO: wait for the retry.
//         Err(FP_NO_IMPL)
//     }


//     /**
//      * Release a page.
//      */
//     pub(crate) fn page_release(&mut self, release_ref: &mut PageRef, flags: BtreeReadFlag) -> FPResult<()> {

//         /* Root should be keep in memory all the time. */
//         if release_ref.is_root() {
//             return Ok(())
//         }

//         if FP_BIT_IST!(self.flags, FP_BTREE_IN_MEMORY) {
//             return Ok(())
//         }


//         Ok(())
//     }

//     /**
//      * Release a reference to a page, and attemppt to immediately evict it.
//      * __wt_page_release_evict
//      */
//     fn page_release_and_evict(&self, release_ref: &mut PageRef, flags: BtreeReadFlag) -> FPResult<()> {

//         let prev_page_ref_state = release_ref.get_state();
//         /* Try require page_ref lock. */
//         let locked = if matches!(prev_page_ref_state, PageRefState::InMemory) && release_ref.cas_state(prev_page_ref_state, PageRefState::Locked){
//             true
//         } else {
//             false
//         };

//         //TODO: release hazard pointer(reference manage for a pageref).
//         //check hazard pointer in the system: __wt_evict > __evict_exclusive > __wt_hazard_check > __wt_session_array_walk
//         if !locked {
//             //TODO: return busy, 
//         }

//         /**
//          * Evict the page.
//          */
//         let evict_flages = 0u32;
//         //TODO: 
//         Err(FP_NO_IMPL)
//     }

// }
// /**
//  * __wt_btree_open -> (__wt_blkcache_open,__wti_btree_tree_open)
//  * creation = ckpt.raw.size == 0;
//  * __btree_tree_open_empty: create a new btree.
//  */
// fn btree_open(ctx: &Context) {

// }

// struct TreeCreateOpt {

// }

// #[cfg(test)]
// mod tests {
//     use crate::{FP_SIZE_OF};

//     use super::*;

//     #[test]
//     fn test_btree_open_tree_create() {
//         // let mut btree = BTree{
//         //     store_oriented: BTreeStoreOriented::Row,
//         // };
//         // btree_open_tree_create(&mut btree)
//     }

//     #[test]
//     fn test_btree_page_alloc() {
//         enum Foo {
//             A(i32),
//             B(i64),
//         }
//         print!("size of Foo: {} bytes", FP_SIZE_OF!(Foo))
//     }

// }

// use std::alloc::{alloc, dealloc, Layout};
// use std::collections::HashMap;
// use std::sync::Mutex;

// lazy_static::lazy_static! {
//     static ref MEMORY_MAP: Mutex<HashMap<*mut u8, Layout>> = Mutex::new(HashMap::new());
// }

// fn allocate(size: usize, align: usize) -> *mut u8 {
//     let layout = Layout::from_size_align(size, align).expect("Invalid layout");
//     let ptr = unsafe { alloc(layout) };
//     if ptr.is_null() {
//         panic!("Memory allocation failed");
//     }
//     MEMORY_MAP.lock().unwrap().insert(ptr, layout);
//     ptr
// }

// fn deallocate(ptr: *mut u8) {
//     let layout = MEMORY_MAP.lock().unwrap().remove(&ptr).expect("Pointer not found");
//     unsafe { dealloc(ptr, layout) };
//     println!("Memory deallocated");
// }

// fn main() {
//     let ptr = allocate(128, 16);
//     println!("Memory allocated at: {:?}", ptr);
//     deallocate(ptr);
// }