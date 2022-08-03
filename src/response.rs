use crate::{
    BlockBodies, BlockHeaders, NewBlock, NewBlockHashes, NewPooledTransactionHashes, NodeData,
    PooledTransactions, Receipts, RequestPair, Status, Transactions,
};

// This type is analogous to the `zebra_network::Response` type.
/// An ethereum network response for version 66.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Response {
    /// The request does not have a response.
    Nil,

    /// The [`Status`](super::Status) message response in the eth protocol handshake.
    Status(Status),

    /// A list of block hashes seen on the network.
    NewBlockHashes(NewBlockHashes),

    /// A new block seen on the network.
    NewBlock(Box<NewBlock>),

    /// A list of transactions seen on the network.
    Transactions(Transactions),

    /// A list of transaction hashes seen on the network.
    NewPooledTransactionHashes(NewPooledTransactionHashes),

    /// The response to a [`Request::GetBlockHeaders`](super::Request::GetBlockHeaders) request.
    BlockHeaders(RequestPair<BlockHeaders>),

    /// The response to a [`Request::GetBlockBodies`](super::Request::GetBlockBodies) request.
    BlockBodies(RequestPair<BlockBodies>),

    /// The response to a [`Request::GetPooledTransactions`](super::Request::GetPooledTransactions) request.
    PooledTransactions(RequestPair<PooledTransactions>),

    /// The response to a [`Request::GetNodeData`](super::Request::GetNodeData) request.
    NodeData(RequestPair<NodeData>),

    /// The response to a [`Request::GetReceipts`](super::Request::GetReceipts) request.
    Receipts(RequestPair<Receipts>),
}
