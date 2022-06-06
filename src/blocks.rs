use anvil_core::eth::block::Header;
use ethers::types::{transaction::eip2718::TypedTransaction, Block, BlockId, Signature};

/// A request for a peer to return block headers starting at the requested block
/// TODO: better comment including limit / skip / reverse rules
pub struct GetBlockHeaders {
    /// The block's number or hash that the peer should start returning headers from
    pub start_block: BlockId,

    /// The maximum number of headers to return
    ///
    /// TODO: should this be limited? does anything denote no limit? if so, this should be a more
    /// expressive type
    pub limit: u64,

    /// The number of blocks that the node should skip while traversing headers to return
    /// TODO: better comment
    pub skip: u32,

    /// Whether or not the headers should be returned in reverse order.
    pub reverse: bool,
}

// TODO: header fastrlp

/// The response to [GetBlockHeaders](crate::GetBlockHeaders), containing headers if any headers were
/// found.
pub struct BlockHeaders(Vec<Header>);

/// A request for a peer to return block bodies for the given block hashes.
pub struct GetBlockBodies(Vec<[u8; 32]>);

// TODO: again, signed message in ethers
/// The response to [GetBlockBodies](crate::GetBlockBodies), containing the block bodies that the
/// peer
pub struct BlockBodies(Vec<Block<(TypedTransaction, Signature)>>);
