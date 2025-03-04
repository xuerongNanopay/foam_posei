#![allow(unused)]

use std::sync::{Arc, Weak};

use crate::{btree::btree::Btree, error::{FP_NO_IMPL, FP_NO_SUPPORT}, internal::FPResult};

use super::{page_header, page_tuple::Tuple, DiskPage, DiskSlice, Page, PageDeleted, PageHeaderV2, PageRef, RefKey};

pub(super) struct InternalPage {
    split_epoch: u64,
    index: InternalIndex,
}

pub(super) struct InternalIndex {
    keys: u32,
    delete_keys: u32,
    index: Vec<PageRef>,
}

impl InternalPage {
    pub(super) fn new_as_row_internal(btree: &Btree, home: Weak<Page>, disk_page: &DiskPage) -> FPResult<Self> {
        let page_header = disk_page.header();
        let mut keys = (page_header.cells_or_flowlen/2) as usize;
        let mut reader = disk_page.cell_reader();

        let mut index = Vec::<PageRef>::with_capacity(keys);
        let mut read_cells = 0u32;
    
        while let (Some(key_tuple), Some(addr_tuple)) = (reader.next(), reader.next()) {
            let mut page_deleted: Option<PageDeleted> = None;
            let mut key: RefKey;
            let mut addr: Option<DiskSlice>;
            let mut state = PageRef::ON_DISK;

            match key_tuple.r#type() {
                Tuple::KEY => {
                    key = RefKey::RowIn(key_tuple.get_disk_data().unwrap());
                    // let page_ref = PageRef::new_with_default(Some(home.clone()), kv.get_tuple_data().unwrap());
                },
                Tuple::KEY_OVFL => {
                    //NEED TODO: bring overflow key to memory. see: __wt_dsk_cell_data_ref_addr.
                    return Err(FP_NO_SUPPORT);
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
                    if page_header.is_set(PageHeaderV2::FAST_TRUNC_UPDATE) {
                        page_deleted = addr_tuple.page_delete();
                    }
                    state = PageRef::DELETED;
                    addr = Some(addr_tuple.get_disk_tuple());

                    if btree.get_modified() {
                        //NEED TODO: make page to dirty.
                    }
                }
                _ => {
                    panic!("Impossible code")
                }
            }

            let page_ref = PageRef::new(Some(home.clone()), key, page_deleted, state);
            index.push(page_ref);
            read_cells += 2;
        };

        Ok(Self {
            split_epoch: 0,
            index: InternalIndex {
                keys: keys as u32,
                delete_keys: 0,
                index,
            },
        })
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
