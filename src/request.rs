use fastrlp::Encodable;

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
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Request {
    /// The [`Status`](super::Status) message sent as part of the eth protocol handshake.
    ///
    /// # Response
    ///
    /// A peer should return a [`Response::Status`](super::Response::Status) in response to complete the
    /// protocol handshake.
    Status(Status),

    /// A list of observed block hashes to be broadcasted.
    ///
    /// # Response
    ///
    /// Returns [`Response::Nil`](super::Response::Nil).
    NewBlockHashes(NewBlockHashes),

    /// A new block to be broadcasted.
    ///
    /// # Response
    ///
    /// Return [`Response::Nil`](super::Response::Nil).
    NewBlock(Box<NewBlock>),

    /// A list of observed transactions to be broadcasted.
    ///
    /// # Response
    ///
    /// Returns [`Response::Nil`](super::Response::Nil).
    Transactions(Transactions),

    /// A list of observed transaction hashes to be broadcasted.
    ///
    /// # Response
    ///
    /// Returns [`Response::Nil`](super::Response::Nil).
    NewPooledTransactionHashes(NewPooledTransactionHashes),

    /// Request block headers from a peer.
    ///
    /// # Response
    ///
    /// Returns [`Response::BlockHeaders`](super::Response::BlockHeaders).
    GetBlockHeaders(RequestPair<GetBlockHeaders>),

    /// Request block bodies from a peer.
    ///
    /// # Response
    ///
    /// Returns [`Response::BlockBodies`](super::Response::BlockBodies).
    GetBlockBodies(RequestPair<GetBlockBodies>),

    /// Request transaction bodies from a peer.
    ///
    /// # Response
    ///
    /// Returns [`Response::PooledTransactions`](super::Response::PooledTransactions).
    GetPooledTransactions(RequestPair<GetPooledTransactions>),

    /// Request state data from a peer.
    ///
    /// # Response
    ///
    /// Returns [`Response::NodeData`](super::Response::NodeData).
    GetNodeData(RequestPair<GetNodeData>),

    /// Request transaction receipts from a peer.
    ///
    /// # Response
    ///
    /// Returns [`Response::Receipts`](super::Response::Receipts).
    GetReceipts(RequestPair<GetReceipts>),
}

/// Encode a [`Request`] into a byte vector, or calculate its RLP length.
///
/// # Example
/// ```rust
/// use ethp2p::{Request, Response};
///
/// let request = Request::Status(Status::default());
/// let mut encoding = vec![];
/// request.encode(&mut encoding).unwrap();
///
/// ```
impl Encodable for Request {
    fn length(&self) -> usize {
        match self {
            Request::Status(status) => status.length(),
            Request::NewBlockHashes(new_block_hashes) => new_block_hashes.length(),
            Request::NewBlock(new_block) => new_block.length(),
            Request::Transactions(transactions) => transactions.length(),
            Request::NewPooledTransactionHashes(new_pooled_transaction_hashes) => {
                new_pooled_transaction_hashes.length()
            }
            Request::GetBlockHeaders(get_block_headers) => get_block_headers.length(),
            Request::GetBlockBodies(get_block_bodies) => get_block_bodies.length(),
            Request::GetPooledTransactions(get_pooled_transactions) => {
                get_pooled_transactions.length()
            }
            Request::GetNodeData(get_node_data) => get_node_data.length(),
            Request::GetReceipts(get_receipts) => get_receipts.length(),
        }
    }

    fn encode(&self, out: &mut dyn bytes::BufMut) {
        match self {
            Request::Status(status) => status.encode(out),
            Request::NewBlockHashes(new_block_hashes) => new_block_hashes.encode(out),
            Request::NewBlock(new_block) => new_block.encode(out),
            Request::Transactions(transactions) => transactions.encode(out),
            Request::NewPooledTransactionHashes(new_pooled_transaction_hashes) => {
                new_pooled_transaction_hashes.encode(out)
            }
            Request::GetBlockHeaders(get_block_headers) => get_block_headers.encode(out),
            Request::GetBlockBodies(get_block_bodies) => get_block_bodies.encode(out),
            Request::GetPooledTransactions(get_pooled_transactions) => {
                get_pooled_transactions.encode(out)
            }
            Request::GetNodeData(get_node_data) => get_node_data.encode(out),
            Request::GetReceipts(get_receipts) => get_receipts.encode(out),
        }
    }
}
