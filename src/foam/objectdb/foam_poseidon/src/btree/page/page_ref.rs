#![allow(unused)]

use std::sync::{atomic::AtomicU8, Arc, Weak};

use super::{DiskSlice, Page};

enum RefKey {
    Col(u64),
    RowOff(Vec<u8>),
    RowOn(DiskSlice)
}

pub(super) struct PageRef {
    home: Option<Weak<Page>>,
    page: Option<Arc<Page>>,
    is_leaf: bool,

    load_state: AtomicU8,
    state: AtomicU8,

    // addr?

    key: RefKey,
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