#![allow(unused)]

use std::sync::{Arc, Weak};

use super::{page_header, page_tuple::Tuple, DiskPage, Page, PageRef, RefKey};

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
    pub(super) fn new_as_row_internal(home: Weak<Page>, disk_page: &DiskPage) {
        let page_header = disk_page.header();
        let mut tuples = (page_header.cells_or_flowlen/2) as usize;
        let mut reader = disk_page.cell_reader();

        let mut index = Vec::<PageRef>::with_capacity(tuples);
        let mut read_cells = 0u32;
    
        while let (Some(key_tuple), Some(addr_tuple)) = (reader.next(), reader.next()) {
            let mut key;
            let mut addr;

            match key_tuple.r#type() {
                Tuple::KEY => {
                    key = RefKey::RowIn(key_tuple.get_disk_data().unwrap());
                    // let page_ref = PageRef::new_with_default(Some(home.clone()), kv.get_tuple_data().unwrap());
                },
                Tuple::KEY_OVFL => {
                    //MUST TODO: bring overflow key to memory. see: __wt_dsk_cell_data_ref_addr.
                    
                },
                _ => {
                    panic!("Impossible code")
                }
            }

            match addr_tuple.r#type() {
                Tuple::ADDR_INTERNAL | Tuple::ADDR_LEAF | Tuple::ADDR_LEAF_NO => {
                    addr = Some(addr_tuple.get_disk_tuple());
                },
                Tuple::ADDR_DEL => {

                }
                _ => {
                    panic!("Impossible code")
                }
            }

            read_cells += 2;
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
