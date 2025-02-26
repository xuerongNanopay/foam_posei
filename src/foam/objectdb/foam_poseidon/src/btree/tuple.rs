#![allow(unused)]

// use crate::{error::{FP_NO_IMPL, FP_NO_SUPPORT}, internal::FPResult, util::compaction::varint, FP_BIT_IST, FP_BIT_MSK, FP_BIT_SET};

// use super::{page::{PageDel}, zone_map::{ZMTxnAddr, ZMTxnValue}};

// /* Descriptor/raw type */
// pub(crate) const FP_BTREE_TUPLE_INLINE_TYPE_MK:u8 = 0x03;
// pub(crate) const FP_BTREE_TUPLE_INLINE_TYPE_ST:u8 = 2;
// pub(crate) const FP_BTREE_TUPLE_INLINE_LEN_MAX:u64 = 63;
// pub(crate) const FP_BTREE_TUPLE_TYPE_MASK:u8 = 0x0f << 4;


// /* Txn Descriptor  */
// pub(crate) const FP_BTREE_TUPLE_TXN_DESC_MK:u8 = 0x08;
// pub(crate) const FP_BTREE_TUPLE_TXN_PREPARE_MK:u8 = 0x01;
// pub(crate) const FP_BTREE_TUPLE_TXN_START_COMMIT_AT_MK:u8 = 0x01 << 1;
// pub(crate) const FP_BTREE_TUPLE_TXN_END_COMMIT_AT_MK:u8 = 0x01 << 2;
// pub(crate) const FP_BTREE_TUPLE_TXN_START_AT_MK:u8 = 0x01 << 3;
// pub(crate) const FP_BTREE_TUPLE_TXN_END_AT_MK:u8 = 0x01 << 4;
// pub(crate) const FP_BTREE_TUPLE_TXN_START_BY_MK:u8 = 0x01 << 5;
// pub(crate) const FP_BTREE_TUPLE_TXN_END_BY_MK:u8 = 0x01 << 6;

// /* Tuple flags. */
// pub(crate) type TupleFlag = u8;
// pub(crate) const FP_BTREE_TUPLE_OVERFLOW_MK:TupleFlag = 0x01;

// #[repr(usize)]
// #[derive(Debug, Clone, Copy, Default)]
// pub(crate) enum TupleType {
//     /**
//      * Key/Value is stored in the TupleHeader.
//      * The size of key or value should not be greater than FP_BTREE_TUPLE_INLINE_MAX.
//      */
//     KeyInline = 0x01,
//     KeyPrefixInline = 0x02,
//     ValueInline = 0x03,
//     /**
//      * Tuple stores address to other page.
//      */
//     #[default]
//     AddrDel = 0x00,
//     AddrInternal = 0x10,
//     AddrLeaf = 0x20,
//     AddrLeafNone = 0x30,
//     /**
//      * Tuple stores key/value.
//      */
//     Del = 0x40,
//     Key = 0x50,
//     KeyOverflow = 0x60,
//     KeyOverflowDel = 0x70,
//     KeyPrefix = 0x80,
//     Value = 0x90,
//     ValueCopy = 0xA0,
//     ValueOverflow = 0xB0,
//     ValueOverflowDel = 0xC0,

// }

// impl TupleType {
//     #[inline(always)]
//     fn to_internal_type(&self) -> TupleType {
//         match self {
//             TupleType::KeyInline | TupleType::KeyPrefixInline | TupleType::KeyPrefix => TupleType::Key,
//             TupleType::ValueInline => TupleType::Value,
//             TupleType::KeyOverflowDel => TupleType::KeyOverflow,
//             TupleType::ValueOverflowDel => TupleType::ValueOverflow,
//             o => o.clone(),
//         }
//     }
// }

// impl TryFrom<&TupleHeader> for TupleType {
//     type Error = ();

//     #[inline(always)]
//     fn try_from(tuple_header: &TupleHeader) -> Result<Self, Self::Error> {
//         if tuple_header.is_inline() {
//             return match tuple_header.descriptor() {
//                 0x01 => Ok(TupleType::KeyInline),
//                 0x02 => Ok(TupleType::KeyPrefixInline),
//                 0x03 => Ok(TupleType::ValueInline),
//                 _ => panic!("impossible tuple type"),
//             };
//         }

//         return match  tuple_header.raw_type() {
//             0x00 => Ok(TupleType::AddrDel),
//             0x10 => Ok(TupleType::AddrInternal),
//             0x20 => Ok(TupleType::AddrLeaf),
//             0x30 => Ok(TupleType::AddrLeafNone),
//             0x40 => Ok(TupleType::Del),
//             0x50 => Ok(TupleType::Key),
//             0x60 => Ok(TupleType::KeyOverflow),
//             0x70 => Ok(TupleType::KeyOverflowDel),
//             0x80 => Ok(TupleType::KeyPrefix),
//             0x90 => Ok(TupleType::Value),
//             0xA0 => Ok(TupleType::ValueCopy),
//             0xB0 => Ok(TupleType::ValueOverflow),
//             0xC0 => Ok(TupleType::ValueOverflowDel),
//             _ => panic!("impossible tuple type"),
//         };
//     }
// }

// /**
//  * On-disk tuple representation.
//  * Each tuple represent an in-line key/value, or address info to get key/value.
//  * Three types of tuple store:
//  *      1. Inline: key/value is embedded inside the tuple header(only for short size key/value).
//  *      2. Normal: key/value is follow the tuple header.
//  *      3. Overflow: key/value is store in seperate page.
//  */
// #[repr(C)]
// #[derive(Debug, Copy, Clone, Default)]
// pub(crate) struct TupleHeader(&'static [u8]);

// impl TupleHeader {
//     #[inline(always)]
//     fn is_key_tuple(&self) -> bool {
//         if self.is_inline() {
//             let t = self.raw_type_inline();
//             t == TupleType::KeyPrefixInline as u8 ||
//             t == TupleType::KeyInline as u8
//         } else {
//             let t = self.raw_type();
//             t == TupleType::Key as u8 ||
//             t == TupleType::KeyOverflow as u8 ||
//             t == TupleType::KeyOverflowDel as u8 ||
//             t == TupleType::KeyPrefix as u8
//         }
//     }

//     #[inline(always)]
//     fn is_value_tuple(&self) -> bool {
//         if self.is_inline() {
//             let t = self.raw_type_inline();
//             t == TupleType::ValueInline as u8
//         } else {
//             let t = self.raw_type();
//             t == TupleType::Value as u8 ||
//             t == TupleType::ValueCopy as u8 ||
//             t == TupleType::ValueOverflow as u8 ||
//             t == TupleType::ValueOverflowDel as u8
//         }
//     }

//     #[inline(always)]
//     fn is_addr_tuple(&self) -> bool {
//         if self.is_inline() {
//             false
//         } else {
//             let t = self.raw_type();
//             t == TupleType::AddrDel as u8 ||
//             t == TupleType::AddrInternal as u8 ||
//             t == TupleType::AddrLeaf as u8 ||
//             t == TupleType::AddrLeafNone as u8
//         }
//     }

//     #[inline(always)]
//     fn is_inline(&self) -> bool {
//         FP_BIT_IST!(self.0[0], FP_BTREE_TUPLE_INLINE_TYPE_MK)
//     }

//     #[inline(always)]
//     fn raw_type_inline(&self) -> u8 {
//         assert!(self.is_inline());

//         let mut ret = self.0[0];
//         FP_BIT_MSK!(ret, FP_BTREE_TUPLE_INLINE_TYPE_MK);
//         ret
//     }

//     #[inline(always)]
//     fn raw_type(&self) -> u8 {
//         assert!(!self.is_inline());

//         let mut ret = self.0[0];
//         FP_BIT_MSK!(ret, FP_BTREE_TUPLE_TYPE_MASK);
//         ret
//     }



//     #[inline(always)]
//     fn enable_key_prefix_comp(&self) -> bool {
//         assert!(!self.is_inline());
//         assert!(self.is_key_tuple());

//         self.raw_type() == TupleType::KeyPrefix as u8
//     }

//     #[inline(always)]
//     fn descriptor(&self) -> u8 {
//         self.0[0]
//     }

//     #[inline(always)]
//     fn prefix_len_inline(&self) -> u8 {
//         assert!(self.is_inline());

//         self.0[1]
//     }

//     #[inline(always)]
//     fn prefix_len(&self) -> u8 {
//         assert!(!self.is_inline());

//         self.0[1]
//     }


//     #[inline(always)]
//     fn inline_data_len(&self) -> usize {
//         assert!(self.is_inline());

//         (self.0[0] >> FP_BTREE_TUPLE_INLINE_TYPE_ST) as usize
//     }

//     /**
//      * Inline tuple do not support transaction description.
//      */
//     #[inline(always)]
//     fn enable_txn_desc(&self) -> bool {
//         assert!(!self.is_inline());
//         assert!(self.is_addr_tuple() || self.is_value_tuple());

//         FP_BIT_IST!(self.0[0], FP_BTREE_TUPLE_TXN_DESC_MK)
//     }

//     #[inline(always)]
//     fn txn_descriptor(&self) -> u8 {
//         assert!(self.enable_txn_desc());

//         self.0[1]
//     }
// }

// impl Into<TupleTxnDesc> for &TupleHeader {
//     #[inline(always)]
//     fn into(self) -> TupleTxnDesc {
//         assert!(self.enable_txn_desc());

//         let (flags, data) = (self.0[1], &self.0[2..]);
//         TupleTxnDesc {
//             flags,
//             data,
//         }
//     }
// }

// #[repr(C)]
// #[derive(Debug, Copy, Clone, Default)]
// pub(crate) struct TupleTxnDesc{
//     data: &'static [u8],
//     flags: u8,
// }

// impl TupleTxnDesc {
//     #[inline(always)]
//     fn has_in_txn_prepare(&self) -> bool {
//         FP_BIT_IST!(self.flags, FP_BTREE_TUPLE_TXN_PREPARE_MK)
//     }

//     #[inline(always)]
//     fn has_txn_start_at(&self) -> bool {
//         FP_BIT_IST!(self.flags, FP_BTREE_TUPLE_TXN_START_AT_MK)
//     }

//     #[inline(always)]
//     fn has_txn_start_by(&self) -> bool {
//         FP_BIT_IST!(self.flags, FP_BTREE_TUPLE_TXN_START_BY_MK)
//     }

//     #[inline(always)]
//     fn has_txn_start_commit_at(&self) -> bool {
//         FP_BIT_IST!(self.flags, FP_BTREE_TUPLE_TXN_START_COMMIT_AT_MK)
//     }

//     #[inline(always)]
//     fn has_txn_end_at(&self) -> bool {
//         FP_BIT_IST!(self.flags, FP_BTREE_TUPLE_TXN_END_AT_MK)
//     }

//     #[inline(always)]
//     fn has_txn_end_commit_at(&self) -> bool {
//         FP_BIT_IST!(self.flags, FP_BTREE_TUPLE_TXN_END_COMMIT_AT_MK)
//     }

//     #[inline(always)]
//     fn has_txn_end_by(&self) -> bool {
//         FP_BIT_IST!(self.flags, FP_BTREE_TUPLE_TXN_END_BY_MK)
//     }
// }

// impl <'a> Into<(ZMTxnAddr, &'a [u8])> for TupleTxnDesc {
//     #[inline(always)]
//     fn into(self) -> (ZMTxnAddr, &'a [u8]) {
//         let mut cur = self.data;
//         let mut val: u64;
//         let mut idx: usize;
//         let mut ret = ZMTxnAddr::new();

//         if self.has_in_txn_prepare() {
//             ret.in_txn_prepare = 1u8;
//         }

//         if self.has_txn_start_at() {
//             (val, idx) = varint::decode_uint(cur).unwrap();
//             ret.oldest_start_at = val;
//             cur = &cur[idx..];
//         }

//         if self.has_txn_start_by() {
//             (val, idx) = varint::decode_uint(cur).unwrap();
//             ret.newest_mod_by = val;
//             cur = &cur[idx..];
//         }

//         if self.has_txn_start_commit_at() {
//             (val, idx) = varint::decode_uint(cur).unwrap();
//             ret.newest_start_commit_at = val + ret.oldest_start_at;
//             cur = &cur[idx..];
//         }

//         if self.has_txn_end_at() {
//             (val, idx) = varint::decode_uint(cur).unwrap();
//             ret.newest_end_at = val + ret.oldest_start_at;
//             cur = &cur[idx..];
//         }

//         if self.has_txn_end_by() {
//             (val, idx) = varint::decode_uint(cur).unwrap();
//             ret.newest_end_by = val;
//             cur = &cur[idx..];
//         }

//         if self.has_txn_end_commit_at() {
//             (val, idx) = varint::decode_uint(cur).unwrap();
//             ret.newest_end_commit_at = val + ret.newest_end_at;
//             cur = &cur[idx..];
//         }

//         (ret, cur)
//     }
// }

// impl <'a> Into<(ZMTxnValue, &'a [u8])> for TupleTxnDesc {
//     #[inline(always)]
//     fn into(self) -> (ZMTxnValue, &'a [u8]) {
//         let mut cur = self.data;
//         let mut val: u64;
//         let mut idx: usize;
//         let mut ret = ZMTxnValue::new();

//         if self.has_in_txn_prepare() {
//             ret.in_txn_prepare = 1u8;
//         }

//         if self.has_txn_start_at() {
//             (val, idx) = varint::decode_uint(cur).unwrap();
//             ret.start_at = val;
//             cur = &cur[idx..];
//         }

//         if self.has_txn_start_by() {
//             (val, idx) = varint::decode_uint(cur).unwrap();
//             ret.start_by = val;
//             cur = &cur[idx..];
//         }

//         if self.has_txn_start_commit_at() {
//             (val, idx) = varint::decode_uint(cur).unwrap();
//             ret.start_commit_at = val + ret.start_at;
//             cur = &cur[idx..];
//         }

//         if self.has_txn_end_at() {
//             (val, idx) = varint::decode_uint(cur).unwrap();
//             ret.end_at = val + ret.start_at;
//             cur = &cur[idx..];
//         }

//         if self.has_txn_end_by() {
//             (val, idx) = varint::decode_uint(cur).unwrap();
//             ret.end_by = val;
//             cur = &cur[idx..];
//         }

//         if self.has_txn_end_commit_at() {
//             (val, idx) = varint::decode_uint(cur).unwrap();
//             ret.end_commit_at = val + ret.end_at;
//             cur = &cur[idx..];
//         }

//         (ret, cur)
//     }
// }


// #[repr(C)]
// pub(crate) enum Tuple {
//     Addr(TupleAddr),
//     Value(TupleValue),
//     Key(TupleKey)
// }

// impl Tuple {
//     /**
//      * __wt_cell_unpack_safe
//      */
//     #[inline(always)]
//     pub(crate) fn new(tuple_header: &TupleHeader) -> FPResult<Tuple> {
//         let descriptor = tuple_header.descriptor();
//         let raw_type = TupleType::try_from(tuple_header).unwrap();
//         let r#type = raw_type.to_internal_type();

//         let mut common = TupleCommon {
//             header: *tuple_header,
//             raw_type,
//             r#type,
//             flags: 0,
//             ..Default::default()
//         };

//         /* Inline tuple */

//         match common.raw_type {
//             TupleType::KeyPrefixInline => {
//                 common.prefix = tuple_header.prefix_len_inline();
//                 common.data = &tuple_header.0[2..tuple_header.inline_data_len()];
//                 common.raw_tuple_len = 2 + tuple_header.inline_data_len();
//                 return Ok(Tuple::Key(TupleKey{
//                     common,
//                 }));
//             },
//             TupleType::KeyInline => {
//                 common.data = &tuple_header.0[1..tuple_header.inline_data_len()];
//                 common.raw_tuple_len = 1 + tuple_header.inline_data_len();
//                 return Ok(Tuple::Key(TupleKey{
//                     common,
//                 }));
//             },
//             TupleType::ValueInline => {
//                 common.data = &tuple_header.0[1..tuple_header.inline_data_len()];
//                 common.raw_tuple_len = 1 + tuple_header.inline_data_len();
//                 return Ok(Tuple::Value(TupleValue{
//                     common,
//                     txn: None,
//                 }));
//             },
//             _ => {},
//         };

//         /* Non-Inline tuple */

//         let mut zm_ta: Option<ZMTxnAddr> = None;
//         let mut zm_tv: Option<ZMTxnValue> = None;
//         let mut cur: &[u8];

//         /* Extract transaction description if exist */
//         if common.header.enable_txn_desc() {
//             match common.r#type {
//                 TupleType::AddrDel | TupleType::AddrInternal | 
//                 TupleType::AddrLeaf | TupleType::AddrLeafNone => {
//                     let txn_desc:TupleTxnDesc = tuple_header.into();
//                     let (zm_txn_addr, rest): (ZMTxnAddr, &[u8])= txn_desc.into();
//                     cur = rest;
//                     zm_ta = Some(zm_txn_addr);
//                 },
//                 TupleType::Del | TupleType::Value | TupleType::ValueCopy |
//                 TupleType::ValueOverflow | TupleType::ValueOverflowDel => {
//                     let txn_desc:TupleTxnDesc = tuple_header.into();
//                     let (zm_txn_value, rest): (ZMTxnValue, &[u8]) = txn_desc.into();
//                     cur = rest;
//                     zm_tv = Some(zm_txn_value);
//                 },
//                 _ => panic!("Tuple new impossible."),
//             };
//         } else if common.header.enable_key_prefix_comp() {
//             common.prefix = tuple_header.prefix_len();
//             cur = &tuple_header.0[2..]
//         } else {
//             cur = &tuple_header.0[1..]
//         }

//         //NEED TODO: fast-truncate.

//         //FEA(Col) TODO: record_number

//         match common.raw_type {
//             TupleType::ValueCopy => {
//                 return  Err(FP_NO_SUPPORT);
//             },
//             /* Overflow key and value. */
//             TupleType::KeyOverflow | TupleType::KeyOverflowDel |
//             TupleType::ValueOverflow | TupleType::ValueOverflowDel |
//             TupleType::AddrDel | TupleType::AddrInternal | TupleType::AddrLeaf | TupleType::AddrLeafNone |
//             TupleType::Key | TupleType::KeyPrefix | TupleType::Value => {

//                 /* Set overflow flag. */
//                 match common.raw_type {
//                     TupleType::KeyOverflow | TupleType::KeyOverflowDel |
//                     TupleType::ValueOverflow | TupleType::ValueOverflowDel => {
//                         FP_BIT_SET!(common.flags, FP_BTREE_TUPLE_OVERFLOW_MK)
//                     },
//                     _ => panic!("Tuple new impossible.")
//                 };

//                 /**
//                  * Tuple head is followed by a 4B data length and data.
//                  */
//                 let (data_len, idx) = varint::decode_uint(cur).unwrap();
//                 cur = &cur[idx..];

//                 let tuple_header_len = unsafe {
//                     let s = tuple_header.0.as_ptr();
//                     let e = cur.as_ptr();
//                     e.offset_from(s)
//                 };

//                 common.raw_tuple_len = tuple_header_len as usize + data_len as usize;
//                 common.data = &cur[..data_len as usize];
//             },
//             _ => panic!("Tuple new error.")
//         };

//         match common.r#type {
//             TupleType::Key | TupleType::KeyOverflow => {
//                 Ok(Tuple::Key(TupleKey{
//                     common
//                 }))
//             },
//             TupleType::Value | TupleType::ValueOverflow => {
//                 Ok(Tuple::Value(TupleValue{
//                     common,
//                     txn: zm_tv,
//                 }))
//             },
//             TupleType::AddrDel | TupleType::AddrInternal | 
//             TupleType::AddrLeaf | TupleType::AddrLeafNone => {
//                 Ok(Tuple::Addr(TupleAddr{
//                     common,
//                     txn: zm_ta,
//                     delete_marker: None,
//                 }))
//             },
//             _ => panic!("Tuple new impossible.")
//         }
//     }
// }

// /**
//  * Common tuple fields.
//  */
// #[repr(C)]
// #[derive(Debug, Copy, Clone, Default)]
// pub(crate) struct TupleCommon {
//     pub(crate) header: TupleHeader,
//     pub(crate) col_v: u64,      /* Run-Length Encoding or Record Number in column-store */

//     pub(crate) data: &'static [u8], /* Data */

//     pub(crate) raw_tuple_len: usize, /* header + data length */

//     pub(crate) prefix: u8,

//     pub(crate) raw_type: TupleType,
//     pub(crate) r#type: TupleType,
//     pub(crate) flags: TupleFlag,
// }

// #[repr(C)]
// pub(crate) struct TupleAddr {
//     pub(crate) common: TupleCommon,
//     pub(crate) txn: Option<ZMTxnAddr>,
//     pub(crate) delete_marker: Option<PageDel>,
// }

// #[repr(C)]
// pub(crate) struct TupleKey {
//     pub(crate) common: TupleCommon,
// }

// #[repr(C)]
// pub(crate) struct TupleValue {
//     pub(crate) common: TupleCommon,
//     pub(crate) txn: Option<ZMTxnValue>,
// }