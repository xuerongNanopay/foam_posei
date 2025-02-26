#![allow(unused)]

// use std::{alloc::Layout, ptr, str::FromStr};

// use crate::{internal::{FPErr, FPResult}, util::ptr::layout_ptr::LayoutPtr, FP_ALLOC, FP_DEALLOC};

// use super::page::{PageIndex, PageRef};


// /**
//  * Row store leaf page.
//  */
// #[repr(C)]
// pub(crate) struct RowLeaf {
//     key: *mut (), /* key in the row store leaf page. */
// }

// /**
//  * In-memory internal key representation.
//  */
// #[repr(C)]
// pub(crate) struct RowKeyMem {
//     layout: Layout,
//     pub(crate) ptr: *mut u8,
//     // offset: u32, /* Key offset in page */
// }

// impl RowKeyMem {
//     pub(crate) fn len(&self) -> usize {
//         return self.layout.size()
//     }
// }

// impl Drop for RowKeyMem {
//     fn drop(&mut self) {
//         FP_DEALLOC!(self.ptr, self.layout);
//     }
// }


// impl FromStr for RowKeyMem {
//     type Err = FPErr;

//     fn from_str(s: &str) -> FPResult<Self> {
//         let (layout, ptr) = FP_ALLOC!(s.len())?;
//         unsafe {
//             ptr::copy_nonoverlapping(s.as_ptr(), ptr, s.len());
//         }
//         Ok(RowKeyMem{layout, ptr})
//     }
// }