#![allow(unused)]

use std::sync::{atomic::{AtomicU8, Ordering}, Arc, Weak};

use crate::{btree::btree::Btree, error::{FP_BTREE_PAGE_ALLOW_RETRY, FP_BTREE_PAGE_NO_FOUND, FP_NO_IMPL}, internal::FPResult, FP_BIT_IST};

use super::{page_tuple::Tuple, DiskSlice, Page, PageDeleted, PageToken, FP_BTREE_PAGE_ADDR_MAX_LENGTH};

pub(super) enum RefKey {
    Col(u64),
    RowOff(Vec<u8>),
    RowIn(DiskSlice)
}

#[derive(Clone, Copy)]
pub(super) enum PageAddrType {
    Internal,
    Leaf,
    LeafNoOverflow,
}


pub(super) struct PageAddr {
    addr: Vec<u8>,
    r#type: PageAddrType,
}

#[derive(Clone, Copy)]
pub(super) struct PageAddrCopy {
    token: PageToken,
    r#type: PageAddrType,
}

pub(super) enum PageRefAddr {
    None,
    In(Tuple),
    Off(PageAddr),
}

pub(super) struct PageRef {
    home: Option<Weak<Page>>,
    page: Option<Arc<Page>>,
    is_leaf: bool,

    load_state: AtomicU8,
    state: AtomicU8,

    key: RefKey,
    //TODO: enum it.
    addr: PageRefAddr,

    page_deleted: Option<PageDeleted>
}

impl PageRef {
    /* loading state. */
    const NO_LOADING:  u8 = 0x00;
    const PREFETCHING: u8 = 0x01;
    const READING:     u8 = 0x02;

    /* state. */
    pub(super) const ON_DISK: u8 = 0x00;
    pub(super) const MARK_DELETED: u8 = 0x01;
    pub(super) const LOCKED:  u8 = 0x02;
    pub(super) const IN_MEM:  u8 = 0x03;
    pub(super) const SPLIT_DEAD:    u8 = 0x04;

    /* read flag */
    pub(super) const READ_IN_MEM:       u32 = 0x01 << 0;
    pub(super) const READ_NO_WAIT:      u32 = 0x01 << 1;
    pub(super) const READ_PREFETCH:     u32 = 0x01 << 2;
    pub(super) const READ_ALLOW_RETRY:  u32 = 0x01 << 3;
}

impl PageRef {
    pub(super) fn new(
        home: Option<Weak<Page>>, 
        key: RefKey,
        page_deleted: Option<PageDeleted>,
        state: u8,
    ) -> Self {
        Self {
            home,
            page: None,
            addr: PageRefAddr::None,
            is_leaf: false,
            state: AtomicU8::new(state),
            load_state: AtomicU8::new(PageRef::NO_LOADING),
            key,
            page_deleted,
        }
    }

    pub(super) fn is_root(&self) -> bool {
        matches!(self.home, None)
    }

    pub(super) fn set_status(&self, new_state: u8) {
        self.state.store(new_state, Ordering::Release);
    }

    pub(super) fn get_status(&self) -> u8 {
        self.state.load(Ordering::Acquire)
    }

    pub(super) fn set_load_status(&self, new_state: u8) {
        self.load_state.store(new_state, Ordering::Release);
    }

    pub(super) fn get_load_status(&self) -> u8 {
        self.load_state.load(Ordering::Acquire)
    }
}

/**
 * Read page.
 */
impl PageRef {
    /**
     * __wt_page_in_func
     */
    fn load_page(&self, btree: &Btree, flags: u32) -> FPResult<()> {
        let mut stalled = false;

        'load_page: loop {
            let state = self.get_status();
            match state {
                PageRef::MARK_DELETED | PageRef::ON_DISK => {
                    match state {
                        PageRef::MARK_DELETED => {
                            if FP_BIT_IST!(flags, PageRef::READ_IN_MEM | PageRef::READ_NO_WAIT) {
                                return Err(FP_BTREE_PAGE_NO_FOUND);
                            }
                        },
                        PageRef::ON_DISK => {
                            if FP_BIT_IST!(flags, PageRef::READ_IN_MEM) {
                                return Err(FP_BTREE_PAGE_NO_FOUND);
                            }
                        },
                        _ => panic!("impossible code"),
                    }

                    //TODO: check if there is enough memory for new page.

                    self.read_page(btree, flags)?;

                    continue 'load_page;
                },
                PageRef::LOCKED => {
                    if FP_BIT_IST!(flags, PageRef::READ_NO_WAIT) {
                        return Err(FP_BTREE_PAGE_NO_FOUND);
                    }

                    if self.get_load_status() == PageRef::READING {
                        if FP_BIT_IST!(flags, PageRef::READ_IN_MEM) {
                            return Err(FP_BTREE_PAGE_NO_FOUND);
                        }

                        // Wait for other thread reading.
                    } else {
                        // Wait for enough memory space.
                    }

                    //NEED TODO: wait logic.
                    continue 'load_page;
                },
                PageRef::SPLIT_DEAD => {
                    return Err(FP_BTREE_PAGE_ALLOW_RETRY);
                },
                PageRef::IN_MEM => {
                    //MUST TODO:
                },
                _ => {
                    panic!("encountered an illegal page ref state: {}", state);
                },
            }
        }

    }

    fn read_page(&self, btree: &Btree, flags: u32) -> FPResult<()> {
        let pre_state = self.get_status();
        
        /* If fails to fetch lock, early return and let caller handle the case. */
        match pre_state {
            PageRef::ON_DISK | PageRef::MARK_DELETED => {
                if self.state.compare_exchange_weak(pre_state, PageRef::LOCKED, Ordering::AcqRel, Ordering::Acquire).is_err() {
                    return Ok(());
                }
                /* lock got, then do the read. */
            }
            _ => {
                return Ok(());
            }
        }

        if pre_state == PageRef::ON_DISK {
            // deleted page need reconciliation.
            self.load_state.store(PageRef::READING, Ordering::Release);
        }

        let addr = self.load_addr(btree)?;

        if matches!(addr, None) {
            return Err(FP_NO_IMPL);
        }

        let addr = addr.unwrap();

        if pre_state == PageRef::MARK_DELETED {
            //NEED TODO: handle delete page.
        }

        /* Read page. */
        btree.read_page(addr.token)?;


        Err(FP_NO_IMPL)
    }

    fn load_addr(&self, btree: &Btree) -> FPResult<Option<PageAddrCopy>> {
        //MUST TODO: lock to protect split.(add free lock in btree.)

        match &self.addr {
            PageRefAddr::None => {
                return Ok(None)
            },
            PageRefAddr::Off(page_addr) => {
                return Ok(Some(PageAddrCopy{
                    token: PageToken::new(&page_addr.addr),
                    r#type: page_addr.r#type,
                }));
            },
            PageRefAddr::In(tuple) => {
                let page_addr = tuple.get_disk_data().unwrap();

                let r#type = match tuple.raw_type() {
                    Tuple::ADDR_INTERNAL => {
                        PageAddrType::Internal
                    },
                    Tuple::ADDR_LEAF => {
                        PageAddrType::Leaf
                    },
                    Tuple::ADDR_DEL => {
                        //MUST TODO
                        return Err(FP_NO_IMPL);
                    },
                    Tuple::ADDR_LEAF_NO => {
                        PageAddrType::LeafNoOverflow
                    }
                    _ => {
                        panic!("impossible code")
                    },
                };

                return Ok(Some(PageAddrCopy{
                    token: PageToken::new(&page_addr),
                    r#type,
                }));
            },
        }

        Err(FP_NO_IMPL)
    }

}


#[cfg(test)]
mod tests {
    use super::*;

    fn is_sync<T: Sync>() {}

    #[test]
    fn test_page() {
        is_sync::<PageRef>();
    }
}