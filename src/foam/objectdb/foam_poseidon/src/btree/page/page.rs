#![allow(unused)]

use std::sync::Arc;

use crate::{error::{FP_BTREE_PAGE_TYPE_ILL, FP_NO_IMPL}, internal::FPResult, FP_BIT_IST, FP_REINTERPRET_CAST_BUF, FP_SIZE_OF};

use super::{page_header, Cell, CellReader, PageHeaderRaw, PageHeaderV2, PageRef, PageSlice, PageType};

pub(super) struct Page {
    header: PageHeaderV2,
    disk: Option<PageDisk>, /* on-disk representation of a page. */

    // inner: PageInner,

}

/**
 * Getter and Setter.
 */
impl Page {
    fn page_type(&self) -> PageType {
        self.header.r#type
    }
}

/**
 * Construct a page from buffer/disk page.
 */
impl Page {
    const HARD_CODE_BLOCK_HEADER_LEN:usize = 28;

    fn new_with_disk_page(disk_page: Vec<u8>) -> FPResult<Self> {
        let page_disk = PageDisk::new(disk_page, 0, FP_SIZE_OF!(PageHeaderRaw) + Page::HARD_CODE_BLOCK_HEADER_LEN)?;
        let page_header = page_disk.header();

        let mut key_cells: u32 = Self::key_cells(&page_disk)?;

        /* Allocate page */
        let mut inner = match page_header.r#type {
            PageType::ColInternal | PageType::RowInternal => {
               let index =  InternalIndex {
                    keys: key_cells,
                    delete_keys: 0,
                    /* TODO: We can Allocate more. */
                    index: Vec::<Arc<PageRef>>::with_capacity(key_cells as usize),
                };

                Inner::Internal(InternalPage{
                    split_epoch: 0,
                    index,
                })
            },
            // PageType::RowLeaf => {

            // },
            _ => return Err(FP_NO_IMPL),
        };

        if let Inner::Internal(page) = &mut inner {

        }

        Ok(Self {
            header: page_header,
            disk: Some(page_disk),
        })
    }

    fn construct_inner(page_raw: &PageDisk, page_header: &PageHeaderV2, key_cells: u32) -> FPResult<Inner> {
        match page_header.r#type {
            PageType::ColInternal | PageType::RowInternal => {
                let internal_index = InternalIndex {
                    keys: key_cells,
                    delete_keys: 0,
                    /* TODO: We can Allocate more. */
                    index: Vec::<Arc<PageRef>>::with_capacity(key_cells as usize),
                };
            },
            PageType::RowLeaf => {

            },
            _ => return Err(FP_NO_IMPL),
        };

        Err(FP_NO_IMPL)
    }

    fn construct_row_internal(page_raw: &PageDisk) {
        let mut reader = page_raw.cell_reader();
        let hint = 0u32;

        while let Some(cell) = reader.next() {
            match cell {
                Cell::KV(kv) => {
                    if matches!(kv.r#type(), Cell::KEY | Cell::KEY_OVFL) {
                    }
                },
                Cell::Addr(addr) => {
                //    match a 
                }
            }
        };
    }

    fn key_cells(page_raw: &PageDisk) -> FPResult<u32> {
        let page_header = page_raw.header();
        let page_tuples = match page_header.r#type {
            PageType::ColLeafVar | PageType::ColLeafFix => {
                page_header.cells_or_flowlen
            },
            PageType::ColInternal => {
                //MUST TODO: check if gap: __wti_page_inmem.
                page_header.cells_or_flowlen
            },
            PageType::RowInternal => {
                page_header.cells_or_flowlen/2
            },
            PageType::RowLeaf => {
                if FP_BIT_IST!(page_header.flags, PageHeaderV2::FLAG_ROW_LEAF_VALUE_EMPTY_ALL) {
                    page_header.cells_or_flowlen
                } else if FP_BIT_IST!(page_header.flags, PageHeaderV2::FLAG_ROW_LEAF_VALUE_EMPTY_NONE) {
                    page_header.cells_or_flowlen/2
                } else {
                    /* Need to interate page to calculate tuple numbers */
                    Self::row_leaf_key_cells(page_raw, &page_header)
                }
            },
            _ => return Err(FP_BTREE_PAGE_TYPE_ILL),
        };
        Ok(page_tuples)
    }

    fn row_leaf_key_cells(page_raw: &PageDisk, page_header: &PageHeaderV2) -> u32 {
        let mut reader = page_raw.cell_reader();
        let mut ret = 0u32;
        while let Some(cell) = reader.next() {
            match cell {
                Cell::KV(c) => {
                    if matches!(c.r#type(), Cell::KEY | Cell::KEY_OVFL) {
                        ret += 1;
                    }
                },
                _ => {
                    panic!("impossible code");
                }
            }
        };
        ret
    }
}

struct PageDisk {
    header_offset: usize,
    cell_offset: usize,
    header: PageHeaderV2,
    raw: PageSlice,
}

impl PageDisk {
    fn new(disk_page: Vec<u8>, header_offset: usize, cell_offset: usize) -> FPResult<Self> {
        let (_, raw_page_header) = PageHeaderRaw::deserialize(&disk_page[..])?;

        let raw = PageSlice::new(disk_page);
        let header = raw_page_header.get_from_raw();

        Ok(Self {
            header_offset,
            cell_offset,
            header,
            raw,
        })
    }

    fn cell_reader(&self) -> CellReader {
        CellReader::new(&self.raw[self.cell_offset..], self.header)
    }

    fn cells(&self) -> &[u8] {
        &self.raw[self.cell_offset..]
    }

    fn header(&self) -> PageHeaderV2 {
        self.header
    }
}

enum Inner {
    Internal(InternalPage),
    RowLeaf(u8),
    ColVar(u8),
    ColFix(u8),
}

struct InternalPage {
    split_epoch: u64,
    // parent
    index: InternalIndex,
}

struct InternalIndex {
    keys: u32,
    delete_keys: u32,
    index: Vec<Arc<PageRef>>,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn is_sync<T: Sync>() {}

    #[test]
    fn test_page() {
        is_sync::<Page>();
    }
}
