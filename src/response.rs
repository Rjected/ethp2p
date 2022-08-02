use crate::{RequestPair, Status, NewBlockHashes, NewBlock, Transactions, NewPooledTransactionHashes, GetBlockHeaders, GetBlockBodies, GetPooledTransactions, GetNodeData, GetReceipts};

// This type is analogous to the `zebra_network::Response` type.
/// An ethereum network response for version 66.
/// TODO: document the response variants.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Response {
    /// The request does not have a response.
    ///
    /// Either:
    ///  * the request does not need a response, or
    ///  * we have no useful data to provide in response to the request,
    ///    or the request does not require any response.
    ///
    /// (Inventory requests provide a list of missing hashes if none of the hashes were available.)
    Nil,

    Status(Status),

    NewBlockHashes(NewBlockHashes),
    NewBlock(Box<NewBlock>),
    Transactions(Transactions),
    NewPooledTransactionHashes(NewPooledTransactionHashes),

    GetBlockHeaders(RequestPair<GetBlockHeaders>),
    GetBlockBodies(RequestPair<GetBlockBodies>),
    GetPooledTransactions(RequestPair<GetPooledTransactions>),
    GetNodeData(RequestPair<GetNodeData>),
    GetReceipts(RequestPair<GetReceipts>),
}
