#![allow(unused)]

use crate::{error::{FP_BTREE_PAGE_TYPE_ILL, FP_NO_IMPL}, internal::FPResult, FP_BIT_IST, FP_REINTERPRET_CAST_BUF, FP_SIZE_OF};

use super::{page_header, Cell, CellReader, PageHeaderRaw, PageHeaderV2, PageType, PageTypeV2};


pub(super) struct Page {
    raw: PageRaw,
    header: PageHeaderV2,
    // inner: PageInner,
}

/**
 * Getter and Setter.
 */
impl Page {
    fn page_type(&self) -> PageTypeV2 {
        self.header.r#type
    }
}

/**
 * Construct a page from buffer/disk page.
 */
impl Page {
    const HARD_CODE_BLOCK_HEADER_LEN:usize = 28;

    fn new_with_raw(raw_page: Vec<u8>) -> FPResult<Self> {
        let raw = PageRaw::new(raw_page, 0, FP_SIZE_OF!(PageHeaderRaw) + Page::HARD_CODE_BLOCK_HEADER_LEN)?;
        let page_header = raw.page_header();

        let mut key_cells: u32 = Self::key_cells(&raw, &page_header)?;

        /* Allocate page */

        Ok(Self {
            header: page_header,
            raw,
        })
    }

    fn construct_inner(page_raw: &PageRaw, page_header: &PageHeaderV2, key_cells: u32) -> FPResult<Inner> {
        match page_header.r#type {
            PageTypeV2::ColInternal | PageTypeV2::RowInternal => {

            },
            _ => return Err(FP_BTREE_PAGE_TYPE_ILL),
        };

        Err(FP_NO_IMPL)
    }

    fn key_cells(page_raw: &PageRaw, page_header: &PageHeaderV2) -> FPResult<u32> {
        let page_header = page_raw.page_header();
        let page_tuples = match page_header.r#type {
            PageTypeV2::ColLeafVar | PageTypeV2::ColLeafFix => {
                page_header.cells_or_flowlen
            },
            PageTypeV2::ColInternal => {
                //MUST TODO: check if gap: __wti_page_inmem.
                page_header.cells_or_flowlen
            },
            PageTypeV2::RowInternal => {
                page_header.cells_or_flowlen/2
            },
            PageTypeV2::RowLeaf => {
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

    fn row_leaf_key_cells(page_raw: &PageRaw, page_header: &PageHeaderV2) -> u32 {
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

struct PageRaw {
    page_header_offset: usize,
    cell_offset: usize,
    raw_page_header: &'static PageHeaderRaw,
    raw_page: Vec<u8>,
}

impl PageRaw {
    fn new(raw_page: Vec<u8>, page_header_offset: usize, cell_offset: usize) -> FPResult<Self> {
        let buffer = &raw_page[..];
        let (size, raw_page_header) = PageHeaderRaw::deserialize(buffer)?;

        let buffer = &buffer[size..];

        Ok(Self {
            page_header_offset,
            cell_offset,
            raw_page_header,
            raw_page,
        })
    }

    fn cell_reader(&self) -> CellReader {
        let page_header = self.raw_page_header.get_from_raw();
        CellReader::new(&self.raw_page[self.cell_offset..], page_header)
    }

    fn raw_page(&self) -> &[u8] {
        &self.raw_page[..]
    }

    fn page_header(&self) -> PageHeaderV2 {
        self.raw_page_header.get_from_raw()
    }
}

enum Inner {
    Internal(u8),
    RowLeaf(u8),
    ColVar(u8),
    ColFix(u8),
}

struct InternalPage {
    split_epoch: u64,
    keys: u32,
    // parent

}

struct InternalIndex {

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_page() {

    }
}
