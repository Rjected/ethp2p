use anvil_core::eth::{block::Block, transaction::TypedTransaction};
use fastrlp::{RlpDecodable, RlpEncodable};
use ruint::Uint;

/// This informs peers of new blocks that have appeared on the network.
#[derive(Clone, Debug, PartialEq, Eq, RlpEncodable, RlpDecodable)]
pub struct NewBlockHashes(pub Vec<BlockHashNumber>);

/// A block hash and a block number.
#[derive(Clone, Debug, PartialEq, Eq, RlpEncodable, RlpDecodable)]
pub struct BlockHashNumber {
    pub hash: [u8; 32],
    pub number: u64,
}

/// A new block with the current total difficult, which includes the difficulty of the returned block.
#[derive(Clone, Debug, PartialEq, Eq, RlpEncodable, RlpDecodable)]
pub struct NewBlock {
    pub block: Block,
    pub td: Uint<128, 2>,
}

// TODO: Introduce TypedTransaction signed message type (with fastrlp encoding) to ethers
/// This informs peers of transactions that have appeared on the network
#[derive(Clone, Debug, PartialEq, Eq, RlpEncodable, RlpDecodable)]
pub struct Transactions(pub Vec<TypedTransaction>);

/// This informs peers of transaction hashes for transactions that have appeared on the network,
/// but have not been included in a block.
#[derive(Clone, Debug, PartialEq, Eq, RlpEncodable, RlpDecodable)]
pub struct NewPooledTransactionHashes(pub Vec<[u8; 32]>);
