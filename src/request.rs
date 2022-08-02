use crate::{
    GetBlockBodies, GetBlockHeaders, GetNodeData, GetPooledTransactions, GetReceipts, NewBlock,
    NewBlockHashes, NewPooledTransactionHashes, RequestPair, Status, Transactions,
};

// This type is analogous to the `zebra_network::Request` type.
/// An ethereum network request for version 66.
///
/// The network layer aims to abstract away the details of the Ethereum wire
/// protocol into a clear request/response API. Each [`Request`] documents the
/// possible [`Response`s](super::Response) it can generate; it is fine (and
/// recommended!) to match on the expected responses and treat the others as
/// `unreachable!()`, since their return indicates a bug in the network code.
/// TODO: document the request variants.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Request {
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
