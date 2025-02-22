#![allow(unused)]

use crate::FP_REINTERPRET_CAST_BUF;

use super::{page_header, PageHeaderRaw};

struct Page {
    inner: PageInner,
}

impl Page {
}

struct PageRaw {

}

struct PageInner {
    page_header_raw: &'static PageHeaderRaw,
    raw_data: Vec<u8>,
}

impl PageInner {
    fn new(raw_data: Vec<u8>) -> Self {
        let buffer = &raw_data[..];
        let (size, page_header_raw) = PageHeaderRaw::deserialize(buffer);

        let buffer = &buffer[size..];

        Self {
            page_header_raw,
            raw_data,
        }
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_page() {

    }
}
