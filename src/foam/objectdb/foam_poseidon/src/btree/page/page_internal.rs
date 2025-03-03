#![allow(unused)]

use std::sync::Arc;

use super::PageRef;

// struct InternalPage {
//     // home: Option<PageDisk>,
//     split_epoch: u64,
//     // parent
//     index: InternalIndex,
// }

// struct InternalIndex {
//     keys: u32,
//     delete_keys: u32,
//     index: Vec<Arc<PageRef>>,
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     fn is_sync<T: Sync>() {}

//     #[test]
//     fn test_page() {
//         is_sync::<InternalPage>();
//     }
// }
