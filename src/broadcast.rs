use ethers::types::{transaction::eip2718::TypedTransaction, Block, Signature};
use fastrlp::{RlpEncodable, RlpDecodable};
use ruint::Uint;

/// This informs peers of new blocks that have appeared on the network.
#[derive(Clone, Debug, PartialEq, Eq, RlpEncodable, RlpDecodable)]
pub struct NewBlockHashes(Vec<BlockHashNumber>);

/// A block hash and a block number.
#[derive(Clone, Debug, PartialEq, Eq, RlpEncodable, RlpDecodable)]
pub struct BlockHashNumber {
    pub hash: [u8; 32],
    pub number: u64,
}

/// A new block with the current total difficult, which includes the difficulty of the returned block.
#[derive(Clone, Debug, PartialEq)]
pub struct NewBlock {
    pub block: Block<(TypedTransaction, Signature)>,
    pub td: Uint<128, 4>,
}

// TODO: Introduce SignedMessage type (with fastrlp encoding) to ethers

/// This informs peers of transactions that have appeared on the network
#[derive(Clone, Debug, PartialEq)]
pub struct Transactions(Vec<(TypedTransaction, Signature)>);

/// This informs peers of transaction hashes for transactions that have appeared on the network,
/// but have not been included in a block.
#[derive(Clone, Debug, PartialEq, Eq, RlpEncodable, RlpDecodable)]
pub struct NewPooledTransactionHashes(Vec<[u8; 32]>);
