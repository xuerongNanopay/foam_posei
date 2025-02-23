#![allow(unused)]

use crate::internal::{FPTimeStamp, FPTxnId};

/**
 * Page marker for delete.
 */
struct PageDeleted {
    txn_id:           FPTxnId,
    timestamp:        FPTimeStamp,
    commit_timestamp: FPTimeStamp,
    is_commited:      bool,
}


struct PageTxnMeta {
    
}