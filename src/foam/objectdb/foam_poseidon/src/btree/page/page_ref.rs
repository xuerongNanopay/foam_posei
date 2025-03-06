#![allow(unused)]

use std::sync::{atomic::{AtomicU8, Ordering}, Arc, Weak};

use crate::{btree::btree::Btree, error::{FP_BTREE_PAGE_ALLOW_RETRY, FP_BTREE_PAGE_NO_FOUND}, internal::FPResult, FP_BIT_IST};

use super::{DiskSlice, Page, PageDeleted};

pub(super) enum RefKey {
    Col(u64),
    RowOff(Vec<u8>),
    RowIn(DiskSlice)
}

pub(super) struct PageRef {
    home: Option<Weak<Page>>,
    page: Option<Arc<Page>>,
    is_leaf: bool,

    load_state: AtomicU8,
    state: AtomicU8,

    key: RefKey,
    //TODO: enum it.
    addr: Option<DiskSlice>,

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
            addr: None,
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

        loop {
            let state = self.get_status();
            match state {
                PageRef::MARK_DELETED => {
                    if FP_BIT_IST!(flags, PageRef::READ_IN_MEM | PageRef::READ_NO_WAIT) {
                        return Err(FP_BTREE_PAGE_NO_FOUND);
                    }
                    //TODO: SKIP DELETE.
                },
                PageRef::ON_DISK => {
                    if FP_BIT_IST!(flags, PageRef::READ_IN_MEM) {
                        return Err(FP_BTREE_PAGE_NO_FOUND);
                    }
                    //TODO: check if there is enough memory for new page.
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

                    stalled = true;
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