#![allow(unused)]

use std::sync::{atomic::AtomicU8, Arc, Weak};

use super::{DiskSlice, Page};

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
    // state: AtomicU8,

    // addr?

    key: RefKey,
    addr: Option<DiskSlice>,
}

impl PageRef {
    /* loading state. */
    const NO_LOADING: u8 = 0x00;
    const PREFETCHING: u8 = 0x01;
    const READING: u8 = 0x02;

    /* state. */
    const ON_DISK: u8 = 0x00;
    const DELETED: u8 = 0x01;
    const LOCKED:  u8 = 0x02;

    pub(super) fn is_root(&self) -> bool {
        matches!(self.home, None)
    }

    pub(super) fn new_with_default(
        home: Option<Weak<Page>>, 
        key: DiskSlice,
    ) -> Self {
        Self {
            home,
            page: None,
            addr: None,
            is_leaf: false,
            key: RefKey::RowIn(key)
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