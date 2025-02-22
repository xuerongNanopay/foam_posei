#![allow(unused)]

use crate::FP_REINTERPRET_CAST_BUF;

use super::{page_header, PageHeaderInner};

struct Page {
    inner: PageInner,
}

impl Page {
    
}

struct PageInner {
    page_header: &'static PageHeaderInner,
    raw_data: Vec<u8>,
}

impl PageInner {
    fn new(raw_data: Vec<u8>) -> Self {
        let page_header = PageHeaderInner::cast_into(&raw_data[..]);
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
