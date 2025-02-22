#![allow(unused)]

use crate::{error::FP_BTREE_PAGE_TYPE_ILL, internal::FPResult, FP_REINTERPRET_CAST_BUF};

use super::{page_header, PageHeaderRaw, PageHeaderV2};

struct Page {
    inner: PageRaw,
    page_header: PageHeaderV2,
}

impl Page {
    fn new(raw_data: Vec<u8>) -> FPResult<Self> {
        let inner = PageRaw::new(raw_data)?;
        let page_header = inner.page_header_raw.get_from_raw();

        match page_header.r#type {

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
    fn new(raw_data: Vec<u8>) -> FPResult<Self> {
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
