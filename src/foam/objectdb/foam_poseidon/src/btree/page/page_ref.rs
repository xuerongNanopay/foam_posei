#![allow(unused)]

use std::sync::{atomic::{AtomicU8, Ordering}, Arc, Weak};

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

    // load_state: AtomicU8,
    state: AtomicU8,

    key: RefKey,
    //TODO: enum it.
    addr: Option<DiskSlice>,

    page_deleted: Option<PageDeleted>
}

impl PageRef {
    /* loading state. */
    const NO_LOADING: u8 = 0x00;
    const PREFETCHING: u8 = 0x01;
    const READING: u8 = 0x02;

    /* state. */
    pub(super) const ON_DISK: u8 = 0x00;
    pub(super) const DELETED: u8 = 0x01;
    pub(super) const LOCKED:  u8 = 0x02;
    pub(super) const IN_MEM:  u8 = 0x03;
    pub(super) const DEAD:    u8 = 0x04;

    pub(super) fn is_root(&self) -> bool {
        matches!(self.home, None)
    }

    pub(super) fn new(
        home: Option<Weak<Page>>, 
        key: DiskSlice,
        page_deleted: Option<PageDeleted>,
        state: u8,
    ) -> Self {
        Self {
            home,
            page: None,
            addr: None,
            is_leaf: false,
            state: AtomicU8::new(state),
            key: RefKey::RowIn(key),
            page_deleted,
        }
    }

    pub(super) fn set_status(&self, new_state: u8) {
        self.state.store(new_state, Ordering::Release);
    }

    pub(super) fn get_status(&self) -> u8 {
        self.state.load(Ordering::Acquire)
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