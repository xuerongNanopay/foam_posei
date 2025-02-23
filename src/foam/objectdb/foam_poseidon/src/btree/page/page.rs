#![allow(unused)]

use crate::{error::FP_BTREE_PAGE_TYPE_ILL, internal::FPResult, FP_BIT_IST, FP_REINTERPRET_CAST_BUF, FP_SIZE_OF};

use super::{page_header, PageHeaderRaw, PageHeaderV2, PageType, PageTypeV2};


struct Page {
    inner: PageRaw,
    page_header: PageHeaderV2,
}

/**
 * Getter and Setter.
 */
impl Page {
    fn page_type(&self) -> PageTypeV2 {
        self.page_header.r#type
    }
}

impl Page {
    const HARD_CODE_BLOCK_HEADER_LEN:usize = 28;

    fn new(raw_data: Vec<u8>) -> FPResult<Self> {
        let inner = PageRaw::new(raw_data, 0, FP_SIZE_OF!(PageHeaderRaw) + Page::HARD_CODE_BLOCK_HEADER_LEN)?;
        let page_header = inner.page_header_raw.get_from_raw();

        let mut page_tuples: u32 = match page_header.r#type {
            PageTypeV2::ColLeafVar | PageTypeV2::ColLeafFix => {
                page_header.entries_or_flowlen
            },
            PageTypeV2::ColInternal => {
                //MUST TODO: check if gap: __wti_page_inmem.
                page_header.entries_or_flowlen
            },
            PageTypeV2::RowInternal => {
                page_header.entries_or_flowlen/2
            },
            PageTypeV2::RowLeaf => {
                if FP_BIT_IST!(page_header.flags, PageHeaderV2::FLAG_ROW_LEAF_VALUE_EMPTY_ALL) {
                    page_header.entries_or_flowlen
                } else if FP_BIT_IST!(page_header.flags, PageHeaderV2::FLAG_ROW_LEAF_VALUE_EMPTY_NONE) {
                    page_header.entries_or_flowlen/2
                } else {
                    /* Need to interate page to calculate tuple numbers */
                    0
                }
            },
            _ => return Err(FP_BTREE_PAGE_TYPE_ILL),
        };

        Ok(Self {
            page_header,
            inner,
        })
    }
}

struct PageRaw {
    page_header_raw: &'static PageHeaderRaw,
    raw_data: Vec<u8>,
}

impl PageRaw {
    fn new(raw_data: Vec<u8>, header_offset: usize, entry_offset: usize) -> FPResult<Self> {
        let buffer = &raw_data[..];
        let (size, page_header_raw) = PageHeaderRaw::deserialize(buffer)?;

        let buffer = &buffer[size..];

        Ok(Self {
            page_header_raw,
            raw_data,
        })
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_page() {

    }
}
