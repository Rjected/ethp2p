use ethers::types::{transaction::eip2718::TypedTransaction, Block, Signature};
use ruint::Uint;

/// This informs peers of new blocks that have appeared on the network.
pub struct NewBlockHashes(Vec<BlockHashNumber>);

/// A block hash and a block number.
pub struct BlockHashNumber {
    pub hash: [u8; 32],
    pub number: u64,
}

/// A new block with the current total difficult, which includes the difficulty of the returned block.
pub struct NewBlock {
    pub block: Block<(TypedTransaction, Signature)>,
    pub td: Uint<128, 4>,
}

// TODO: Introduce SignedMessage type (with fastrlp encoding) to ethers

/// This informs peers of transactions that have appeared on the network
pub struct Transactions(Vec<(TypedTransaction, Signature)>);

/// This informs peers of transaction hashes for transactions that have appeared on the network,
/// but have not been included in a block.
/// TODO: question: are there limits to how many hashes the client stores? The peer broadcasting
/// this message is not guaranteed to have the hash preimage, so clients need limits on hashes they
/// will request. A peer could respond to a GetPooledTransactions message with an empty list, and
/// that would be valid. Could this allow peers to corrupt client tx hash pools with fake tx hashes?
pub struct NewPooledTransactionHashes(Vec<[u8; 32]>);
