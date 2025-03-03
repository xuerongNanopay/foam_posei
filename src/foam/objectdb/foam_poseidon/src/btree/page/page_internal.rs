#![allow(unused)]

use std::sync::Arc;

use super::{page_tuple::Tuple, page_header, DiskPage, PageRef};

pub(super) struct InternalPage {
    // home: Option<PageDisk>,
    split_epoch: u64,
    // parent
    index: InternalIndex,
}

pub(super) struct InternalIndex {
    keys: u32,
    delete_keys: u32,
    index: Vec<PageRef>,
}

impl InternalPage {
    pub(super) fn new_as_row_internal(disk_page: &DiskPage) {
        let page_header = disk_page.header();
        let cells = page_header.cells_or_flowlen/2;
        let mut reader = disk_page.cell_reader();

        let index = Vec::<PageRef>::with_capacity(cells as usize);

        while let Some(cell) = reader.next() {
            match cell {
                Tuple::KV(kv) => {
                    if matches!(kv.r#type(), Tuple::KEY | Tuple::KEY_OVFL) {
                    }
                },
                _ => {
                    panic!("Impossible code")
                }
            }
        };
    }

    
}

#[cfg(test)]
mod tests {
    use super::*;

    fn is_sync<T: Sync>() {}

    #[test]
    fn test_page() {
        is_sync::<InternalPage>();
    }
}
