#![allow(unused)]

use crate::FP_REINTERPRET_CAST_BUF;

use super::{page_header, PageHeaderRaw};

struct Page {
}

impl Page {
    
}

struct PageRaw {
    page_header: &'static PageHeaderRaw,
    raw_data: Vec<u8>,
}

impl PageRaw {
    fn new(raw_data: Vec<u8>) -> Self {
        let page_header = PageHeaderRaw::cast_into(&raw_data[..]);
        Self {
            page_header,
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
