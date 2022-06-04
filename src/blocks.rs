use anvil_core::eth::block::Header;
use ethers::types::{transaction::eip2718::TypedTransaction, Block, BlockId, Signature};

// GetBlockHeaders(),
// BlockHeaders(),
// GetBlockBodies(),
// BlockBodies(),

/// A request for a peer to return block headers starting at the requested block
/// TODO: better comment
pub struct GetBlockHeaders {
    /// The block's number or hash that the peer should start returning headers from
    pub start_block: BlockId,

    /// The maximum number of headers to return
    ///
    /// TODO: should this be limited? does anything denote no limit?
    pub limit: u64,

    /// The number of blocks that the node should skip while traversing headers to return
    /// TODO: better comment
    /// TODO: question: why should nodes request a large skip? or any at all?
    /// would nodes just go searching for headers?
    /// would large skips increase resource usage by nodes?
    pub skip: u32,

    /// Whether or not the headers should be returned in reverse order.
    pub reverse: bool,
}

/// The response to [GetBlockHeaders](crate::GetBlockHeaders), containing headers if any headers were
/// found.
/// TODO: question: what if a node returns headers in the wrong order? How much work will the
/// recipient do before determining that the message was invalid? again, why use this skip
/// functionality? what optimization is being enabled and how is it implemented? under untrusted
/// input is it free from DoS vulnerabilities?
/// also TODO: header fastrlp
pub struct BlockHeaders(Vec<Header>);

/// A request for a peer to return block bodies for the given block hashes.
pub struct GetBlockBodies(Vec<[u8; 32]>);

/// The response to [GetBlockBodies](crate::GetBlockBodies), containing the block bodies that the
/// peer
/// TODO: question: again, what if malicious node? how do we check net messages like this -
/// untrusted
/// TODO: again, signed message in ethers
pub struct BlockBodies(Vec<Block<(TypedTransaction, Signature)>>);
