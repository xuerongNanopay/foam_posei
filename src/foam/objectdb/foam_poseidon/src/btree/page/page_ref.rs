#![allow(unused)]

use std::{rc::Weak, sync::{atomic::AtomicU8, Arc}};

use super::PageV2;

enum RefKey {
    Col(u64),
    Row(&'static [u8]),
}

struct PageRef {
    page: Option<Arc<PageV2>>,
    home: Weak<PageV2>,
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
}