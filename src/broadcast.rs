use ethers::types::{transaction::eip2718::TypedTransaction, Block, Signature};
use ruint::Uint;

/// This informs peers of new blocks that have appeared on the network.
pub struct NewBlockHashes(Vec<BlockHashNumber>);

/// A block hash and a block number.
pub struct BlockHashNumber {
    pub hash: [u8; 32],
    pub number: u64,
}

/// A new block with the current total difficulty.
pub struct NewBlock {
    pub block: Block<(TypedTransaction, Signature)>,
    pub td: Uint<128, 4>,
}

// TODO: Introduce SignedMessage type (with fastrlp encoding) to ethers

/// TODO: comment
pub struct Transactions(Vec<(TypedTransaction, Signature)>);

/// TODO: comment
pub struct NewPooledTransactionHashes(Vec<[u8; 32]>);
