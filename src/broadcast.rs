use anvil_core::eth::{block::Block, transaction::TypedTransaction};
use fastrlp::{RlpDecodable, RlpDecodableWrapper, RlpEncodable, RlpEncodableWrapper};
use ruint::Uint;

/// This informs peers of new blocks that have appeared on the network.
#[derive(Clone, Debug, PartialEq, Eq, RlpEncodableWrapper, RlpDecodableWrapper)]
pub struct NewBlockHashes(
    /// New block hashes and the block number for each blockhash.
    /// Clients should request blocks using a [`GetBlockBodies`](crate::GetBlockBodies) message.
    pub Vec<BlockHashNumber>
);

/// A block hash _and_ a block number.
#[derive(Clone, Debug, PartialEq, Eq, RlpEncodable, RlpDecodable)]
pub struct BlockHashNumber {
    /// The block hash
    pub hash: [u8; 32],
    /// The block number
    pub number: u64,
}

/// A new block with the current total difficulty, which includes the difficulty of the returned
/// block.
#[derive(Clone, Debug, PartialEq, Eq, RlpEncodable, RlpDecodable)]
pub struct NewBlock {
    /// A new block.
    pub block: Block,
    /// The current total difficulty.
    pub td: Uint<128, 2>,
}

/// This informs peers of transactions that have appeared on the network and are not yet included
/// in a block.
#[derive(Clone, Debug, PartialEq, Eq, RlpEncodableWrapper, RlpDecodableWrapper)]
pub struct Transactions(
    /// New transactions for the peer to include in its mempool.
    pub Vec<TypedTransaction>
);

impl From<Vec<TypedTransaction>> for Transactions {
    fn from(txs: Vec<TypedTransaction>) -> Self {
        Transactions(txs)
    }
}

impl From<Transactions> for Vec<TypedTransaction> {
    fn from(txs: Transactions) -> Self {
        txs.0
    }
}

/// This informs peers of transaction hashes for transactions that have appeared on the network,
/// but have not been included in a block.
#[derive(Clone, Debug, PartialEq, Eq, RlpEncodableWrapper, RlpDecodableWrapper)]
pub struct NewPooledTransactionHashes(
    /// Transaction hashes for new transactions that have appeared on the network.
    /// Clients should request the transactions with the given hashes using a
    /// [`GetPooledTransactions`](crate::GetPooledTransactions) message.
    pub Vec<[u8; 32]>
);
