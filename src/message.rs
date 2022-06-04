use crate::{
    blocks::{BlockBodies, BlockHeaders, GetBlockBodies},
    broadcast::{NewBlock, NewBlockHashes, NewPooledTransactionHashes, Transactions},
    GetBlockHeaders, GetNodeData, GetPooledTransactions, GetReceipts, NodeData, PooledTransactions,
    Receipts, Status,
};

// TODO: determine whats up with this enum variant size warning

/// Represents a message in the eth wire protocol, versions 65 and 66.
///
/// The ethereum wire protocol is a set of messages that are broadcasted to the network in two
/// styles:
///  * A request message sent by a peer (such as `GetPooledTransactions`), and an associated response message (such as `PooledTransactions`).
///  * A message that is broadcast to the network, without a corresponding request.
///
///  The newer eth/66 is an efficiency upgrade on top of eth/65, introducing a request id to correlate
///  request-response message pairs.
pub enum EthMessage {
    // Status is required for the protocol handshake
    Status(Status),

    // The following messages are broadcast to the network
    NewBlockHashes(NewBlockHashes),
    NewBlock(NewBlock),
    Transactions(Transactions),
    NewPooledTransactionHashes(NewPooledTransactionHashes),

    // The following messages are request-response message pairs
    GetBlockHeaders(RequestPair<GetBlockHeaders>),
    BlockHeaders(RequestPair<BlockHeaders>),
    GetBlockBodies(RequestPair<GetBlockBodies>),
    BlockBodies(RequestPair<BlockBodies>),
    GetPooledTransactions(RequestPair<GetPooledTransactions>),
    PooledTransactions(RequestPair<PooledTransactions>),
    GetNodeData(RequestPair<GetNodeData>),
    NodeData(RequestPair<NodeData>),
    GetReceipts(RequestPair<GetReceipts>),
    Receipts(RequestPair<Receipts>),
}

/// This represents a network message which has a request id
pub struct RequestPair<T> {
    /// id for the contained request or response message
    pub request_id: u64,

    /// the request or response message payload
    pub message: T,
}

// TODO: implement rlp encoding/decoding for RequestPair<T> where T: rlp encodable / decodable
// TODO: question: is encoding / decoding for Vec<T> where T: encodable / decodable auto implemented?
