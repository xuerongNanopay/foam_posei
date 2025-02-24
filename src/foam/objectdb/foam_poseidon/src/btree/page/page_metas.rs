#![allow(unused)]

use crate::internal::{FPTimeStamp, FPTxnId};

/**
 * Page marker for delete.
 */
pub(crate) struct PageDeleted {
    txn_id:           FPTxnId,
    timestamp:        FPTimeStamp,
    commit_timestamp: FPTimeStamp,
    is_commited:      bool,
}

/**
 * Work with MVCC
 */
pub(crate) struct PageAddrTS {
    initial_commit_ts: FPTimeStamp,
    end_commit_ts: FPTimeStamp,
}

/**
 * Work with MVCC
 */
pub(crate) struct PageKVTS {

}