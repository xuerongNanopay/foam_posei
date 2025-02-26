#![allow(unused)]

use std::{cell::UnsafeCell, ptr, sync::Arc};

// use crate::{btree::{lex_prefix_cmp, lex_skip_cmp, page::PageIndex, BtreeInsert, BtreeInsertList, BtreeReadFlag, FP_BTEE_READ_NEED_ONCE, FP_BTEE_READ_RETRY_OK, FP_BTREE_LEX_PREFIX_CMP_MAX_LEN, FP_BTREE_MAX_KV_SIZE, FP_RECORD_NUMBER_OOB}, cursor::{CursorFlag, CursorItem, ICursor, CURSOR_BOUND_LOWER, CURSOR_BOUND_LOWER_INCLUSIVE, CURSOR_BOUND_UPPER, CURSOR_BOUND_UPPER_INCLUSIVE}, error::{FP_NO_IMPL, FP_NO_SUPPORT}, internal::FPResult, util::ptr::layout_ptr::LayoutPtr, FP_BIT_CLR, FP_BIT_IST, FP_BIT_SET, FP_MIN};

// use super::{BTree, BTreeType, Page, PageRef, PageType};

// struct BtreeCursorState {
//     key: CursorItem,
//     value: CursorItem,
//     record_number: u64,
//     flags: CursorFlag
// }

// impl BtreeCursorState {
//     fn from_cursor(cursor: &ICursor) -> BtreeCursorState {
//         BtreeCursorState {
//             record_number: cursor.record_number,
//             flags: cursor.flags,
//             key: cursor.key.clone(),
//             value: cursor.value.clone(),
//         }
//     }
// }

// pub(crate) type BtreeCursorFlag = u32;
// pub const FP_BTREE_CURSOR_ACTIVE:               BtreeCursorFlag = 1 << 0;
// pub const FP_BTREE_CURSOR_ITER_APPEND:          BtreeCursorFlag = 1 << 2;  /* Column store: iterating append list. */
// pub const FP_BTREE_CURSOR_ITER_NEXT:            BtreeCursorFlag = 1 << 3; 
// pub const FP_BTREE_CURSOR_ITER_PREV:            BtreeCursorFlag = 1 << 4; 
// pub const FP_BTREE_CURSOR_ITER_RETRY_NEXT:      BtreeCursorFlag = 1 << 5;
// pub const FP_BTREE_CURSOR_ITER_RETRY_PREV:      BtreeCursorFlag = 1 << 6;
// pub const FP_BTREE_CURSOR_READ_NEED_ONCE:            BtreeCursorFlag = 1 << 7;  /* control FP_BTEE_TRAV_ONCE */


// pub const FP_BTREE_CURSOR_POSITION_MASK: BtreeCursorFlag = 
//     FP_BTREE_CURSOR_ITER_APPEND | 
//     FP_BTREE_CURSOR_ITER_NEXT |
//     FP_BTREE_CURSOR_ITER_PREV |
//     FP_BTREE_CURSOR_ITER_RETRY_NEXT | 
//     FP_BTREE_CURSOR_ITER_RETRY_PREV;

// /**
//  * Btree cursor.
//  */
// pub(crate) struct BtreeCursor<'a> {
//     pub(crate) icur: &'a ICursor,
//     btree: UnsafeCell<BTree>,

//     // Date Source.
//     pub(crate) data_source: *mut(),

//     /* Pointer may be better options */
//     pub(crate) cur_page: *mut PageRef,
//     pub(crate) slot: u32,
    
//     pub(crate) record_number: u64,

//     pub(crate) cur_insert_list: *mut BtreeInsertList,
//     pub(crate) cur_insert: *mut BtreeInsert,

//     pub(crate) flags: BtreeCursorFlag,

//     pub(crate) page_ref: *mut PageRef,
// }

// impl BtreeCursor<'_,> {
//     pub fn get_tree(&self) -> &BTree {
//         unsafe {
//             &*self.btree.get()   
//         }
//     }
//     pub fn get_tree_mut(&self) -> &mut BTree {
//         unsafe {
//             &mut *self.btree.get()   
//         }
//     }

//     /**
//      * __wt_btcur_insert
//      */
//     pub(crate) fn insert(&mut self) -> FPResult<()> {

//         let mut save_state: BtreeCursorState;

//         //TODO: stats
//         let insert_len = self.icur.key.len() + self.icur.value.len();

//         // Verify KV length.
//         if matches!(self.get_tree().r#type, BTreeType::Row) {
//             self.size_check(&self.icur.key)?;
//         }
//         self.size_check(&self.icur.value)?;

//         // Bulk-load only available before the first insert.
//         // self.btree_dao.get_btree().disable_bulk_load();

//         //TODO: support append for column stored.

//         //TODO: Override.

//         /* Verify bounds. */
//         self.is_key_within_bounds()?;

//         save_state = self.save_cursor_state();

//         self.init(true);

//         match self.get_tree().r#type {
//             BTreeType::Row => {
//                 //1. search row.
//                 //2. fail if duplicate if duplicate not allow.
//                 //3. modify row.
//             },
//             _ => {
//                 return Err(FP_NO_IMPL)
//             }
//         }

//         Ok(())
//     }

//     /**
//      * Check if a insert item is too large. 
//      */
//     fn size_check(&self, item: &CursorItem) -> FPResult<()> {

//         if !matches!(self.get_tree().r#type, BTreeType::ColumnFix) {
//             //see: __cursor_size_chk
//             panic!("Do not support")
//         }

//         if item.len() <= FP_GIGABYTE {
//             return Ok(())
//         }

//         if item.len() > FP_BTREE_MAX_KV_SIZE {
//             return Err(FP_NO_SUPPORT)
//         }

//         /* Check if block hanlder can wactually write. */
//         // self.btree_dao.write_size(item.len())?;

//         Ok(())
//     }

//     /**
//      * Verify if a given key is in within the bounds.
//      */
//     fn is_key_within_bounds(&self) -> FPResult<bool> {

//         let mut ret= false;

//         /* Ignore if not config */
//         if FP_BIT_IST!(self.icur.flags, CURSOR_BOUND_LOWER | CURSOR_BOUND_UPPER){
//             return Ok(true);
//         }

//         //TODO: unlikely assert.

//         if FP_BIT_IST!(self.icur.flags, CURSOR_BOUND_LOWER) && false {
//             ret = self.compare_bounds(false)?;
//         }

//         if FP_BIT_IST!(self.icur.flags, CURSOR_BOUND_UPPER) {
//             ret = self.compare_bounds(true)?;
//         }

//         Ok(ret)
//     }

//     fn compare_bounds(&self, upper: bool) -> FPResult<bool> {
//         let mut record_number_bound: u64;
//         let mut cmp: i32;
//         if upper {
//             if matches!(self.get_tree().r#type, BTreeType::Row) {
//                 //TODO: investigate key_cmp_fn.
//                 cmp = self.get_tree().key_cmp_fn.as_ref().unwrap().compare((self.icur.key.data.as_ptr(), self.icur.key.len()), (self.get_tree().upper_bound.data.as_ptr(), self.get_tree().upper_bound.len()));
//             } else {
//                 //TODO: column.
//                 return Err(FP_NO_IMPL);
//             }

//             if FP_BIT_IST!(self.icur.flags, CURSOR_BOUND_UPPER_INCLUSIVE) {
//                 if matches!(self.get_tree().r#type, BTreeType::Row) {
//                     return Ok(cmp > 0);
//                 } else {
//                     return Err(FP_NO_IMPL);
//                 }
//             } else {
//                 if matches!(self.get_tree().r#type, BTreeType::Row) {
//                     return Ok(cmp >= 0);
//                 } else {
//                     return Err(FP_NO_IMPL);
//                 }
//             }
//         } else {
//             if matches!(self.get_tree().r#type, BTreeType::Row) {
//                 //TODO: investigate key_cmp_fn.
//                 cmp = self.get_tree().key_cmp_fn.as_ref().unwrap().compare((self.icur.key.data.as_ptr(), self.icur.key.len()), (self.get_tree().lower_bound.data.as_ptr(), self.get_tree().lower_bound.len()));

//             } else {
//                 return Err(FP_NO_IMPL);
//             }

//             if FP_BIT_IST!(self.icur.flags, CURSOR_BOUND_LOWER_INCLUSIVE) {
//                 if matches!(self.get_tree().r#type, BTreeType::Row) {
//                     return Ok(cmp < 0);
//                 } else {
//                     return Err(FP_NO_IMPL);
//                 }
//             } else {
//                 if matches!(self.get_tree().r#type, BTreeType::Row) {
//                     return Ok(cmp <= 0);
//                 } else {
//                     return Err(FP_NO_IMPL);
//                 }
//             }
//         }
//         Ok(false)
//     }

//     /**
//      * Initial cursor.
//      */
//     fn init(&mut self, reenter: bool) -> FPResult<()> {

//         if reenter {
//             self.reset()?;
//         }


//         Ok(())
        
//     }

//     /**
//      * Reset cursor.
//      * __cursor_reset
//      */
//     fn reset(&mut self) -> FPResult<()> {
//         self.clear_position()?;

//         /* Deactive the cursor. */
//         if FP_BIT_IST!(self.flags, FP_BTREE_CURSOR_ACTIVE) {
//             FP_BIT_CLR!(self.flags, FP_BTREE_CURSOR_ACTIVE)
//         }

//         if self.page_ref.is_null() {
//             return Ok(())
//         }

//         //TODO: more
//         Ok(())
//     }

//     /**
//      * Clear position.
//      */
//     fn clear_position(&mut self) -> FPResult<()> {
//         self.record_number = FP_RECORD_NUMBER_OOB;

//         self.cur_insert = ptr::null_mut();
//         self.cur_insert_list = ptr::null_mut();

//         FP_BIT_CLR!(self.flags, FP_BTREE_CURSOR_POSITION_MASK);

//         Ok(())
//     }

//     /**
//      * Row-store search from a cursor.
//      * __cursor_row_search, __wt_row_search
//      */
//     fn row_search(&mut self, insert: bool, leaf_safe: bool) -> FPResult<Option<*mut PageRef>> {
//         //TODO: page lock/resource generation manage.

//         // let mut current: *mut PageRef = ptr::null_mut();

//         self.clear_position();

//         //TODO: support column append.

//         /* Search b-tree from the root */
//         let btree = self.get_tree_mut();
//         let mut current = &mut self.get_tree_mut().root;
//         let mut pindex: Option<&PageIndex> = None;
//         let mut parent_pindex: Option<&PageIndex> = None;
//         let mut page: &LayoutPtr<Page>;
//         // let current = &mut self.get_tree().root as *mut PageRef;
//         // let mut pindex: *mut PageIndex = ptr::null_mut();
//         // let mut parent_pindex: *mut PageIndex = ptr::null_mut();
//         let depth: i32 = 2;

//         let search_key = &self.icur.key;

//         let mut descent: &PageRef;
//         let mut read_flags: BtreeReadFlag;

//         loop {
//             //TODO: release current page.
//             btree.page_release( current, 0);
//             // parent_pindex = pindex;
//             page = current.page.as_ref().unwrap();

//             if !matches!(page.r#type, PageType::Internal) {
//                 break;
//             }

//             let page_index = page.get_page_index()?;

//             //TODO: append.

//             /**
//              * Binary search on internal page.
//              *  1. lexicograpohic order short/prefix.
//              *  2. lexicograpohic order.
//              *  3. custom key order.
//              * 
//              * Internal indexex layout:
//              *              [index: 0]    [index: 1]      [index: 2]      [index: 3]
//              *       (-infinite, key1)    [key1, key2)    [key2, key3)    [key3, +infinite]
//              */
//             let mut base = 1u32;
//             let mut limit = page_index.entries - 1;
//             let mut descent_key_match = false;

//             if btree.key_cmp_fn.is_none() && search_key.len() <= FP_BTREE_LEX_PREFIX_CMP_MAX_LEN {
//                 /* Lexicographic order for short search key. */
//                 while limit != 0 {
//                     let item: CursorItem;
//                     let idx = base + (limit>>1);
//                     let descent = unsafe {
//                         &**page_index.indexes.add(idx as usize)
//                     };

//                     let (pkey, skey) = descent.get_ref_key()?;

//                     let cmp = lex_prefix_cmp((search_key.data.as_ptr(), search_key.len()), (pkey as * const u8, skey));
//                     if cmp > 0 {
//                         base = idx + 1;
//                         limit -= 1;
//                     } else if cmp == 0 {
//                         descent_key_match = true;
//                         break;
//                     }

//                     limit >>= 1;
//                 }
//             }
//             else if btree.key_cmp_fn.is_none() {
//                 /**
//                  * Lexicographic order for regular search key.
//                  * Use skip_high and skip_low to skip common portion.
//                  * The algorithm will/may improve the performance after second looping for binary seach,
//                  * because the low, high boundary can only be determine after second loop.
//                  * Once the skip_low and skip_high are known, we can use is to skip common prefix in comparison.
//                  * However, if the search key are close to the low and high boundaries,
//                  * the idea will not improve the performance a lot.
//                  *  
//                  * eg: 
//                  *  idea scenario: improve key comparison performance, as boundaries will determine after second binary search loop):
//                  *  low(AAAA) |------------------------------------------| high(ZZZZZZZ)
//                  *                          ^[search key]
//                  * 
//                  *  zero-effect scenario: as skip_low will always be zero during binary search.
//                  *  low(AAAA) |------------------------------------------| high(ZZZZZZZ)
//                  *            ^[search key]
//                  * 
//                  *  zero-effect scenario: as skip_high will always be zero during binary search.
//                  *  low(AAAA) |------------------------------------------| high(ZZZZZZZ)
//                  *                                                       ^[search key]
//                  */
//                 let mut skip_high = 0usize;
//                 let mut skip_low = 0usize;
//                 let mut skip;

//                 let mut base = 1u32;
//                 let mut limit = page_index.entries - 1;

//                 while limit != 0 {

//                     let item: CursorItem;
//                     let idx = base + (limit>>1);
//                     let descent = unsafe {
//                         &**page_index.indexes.add(idx as usize)
//                     };

//                     skip = FP_MIN!(skip_high, skip_low);
//                     let (pkey, skey) = descent.get_ref_key()?;

//                     let cmp = lex_skip_cmp((search_key.data.as_ptr(), search_key.len()), (pkey as * const u8, skey), &mut skip);
//                     if cmp > 0 {
//                         skip_low = skip;
//                         base = idx + 1;
//                         limit -= 1;
//                     } else if cmp > 0 {
//                         skip_high = skip;
//                     } else {
//                         descent_key_match = true;
//                         break;
//                     }

//                     limit >>= 1;
//                 }
//             } else {
//                 while limit != 0 {
//                     let item: CursorItem;
//                     let idx = base + (limit>>1);
//                     let descent = unsafe {
//                         &**page_index.indexes.add(idx as usize)
//                     };

//                     let (pkey, skey) = descent.get_ref_key()?;

//                     let cmp_fn = btree.key_cmp_fn.as_ref().unwrap();
                    

//                     let cmp = lex_prefix_cmp((search_key.data.as_ptr(), search_key.len()), (pkey as * const u8, skey));
//                     if cmp > 0 {
//                         base = idx + 1;
//                         limit -= 1;
//                     } else if cmp == 0 {
//                         descent_key_match = true;
//                         break;                    }

//                     limit >>= 1;
//                 }
//             }

//             /**
//              * If no found, the descend will be base-1;
//              */
//             if !descent_key_match {
//                 descent = unsafe {
//                     &**page_index.indexes.add((base-1) as usize)
//                 };
//             }

//             //TODO: right side decend.

//             /**
//              * key is greater than any key in the page.
//              */
//             if page_index.entries == base {
//                 //TODO:
//             }

//             /* descend */

//             read_flags = FP_BTEE_READ_RETRY_OK;
//             if FP_BIT_IST!(self.flags, FP_BTREE_CURSOR_READ_NEED_ONCE) {
//                 FP_BIT_SET!(read_flags, FP_BTEE_READ_NEED_ONCE)
//             }

//         }

//         Ok(None)
//     }

//     fn save_cursor_state(&self) -> BtreeCursorState {
//         BtreeCursorState::from_cursor(&self.icur)
//     }
// }