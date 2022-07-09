mod broadcast;
pub use broadcast::{
    BlockHashNumber, NewBlock, NewBlockHashes, NewPooledTransactionHashes, Transactions,
};

mod message;
pub use message::{EthMessage, EthMessageID, ProtocolMessage, RequestPair};

mod status;
pub use status::Status;

mod blocks;
pub use blocks::{BlockBodies, BlockHashOrNumber, BlockHeaders, GetBlockBodies, GetBlockHeaders};

mod transactions;
pub use transactions::{GetPooledTransactions, PooledTransactions};

mod receipts;
pub use receipts::{GetReceipts, Receipts};

mod state;
pub use state::{GetNodeData, NodeData};

mod version;
pub use version::EthVersion;

// impl from for each variant of EthMessage
macro_rules! message_from_impl {
    ($t:ty, $variant:ident) => {
        impl From<$t> for EthMessage {
            fn from(t: $t) -> Self {
                EthMessage::$variant(t)
            }
        }
    };
}

message_from_impl!(Status, Status);
message_from_impl!(NewBlockHashes, NewBlockHashes);
message_from_impl!(Box<NewBlock>, NewBlock);
message_from_impl!(Transactions, Transactions);
message_from_impl!(NewPooledTransactionHashes, NewPooledTransactionHashes);
message_from_impl!(RequestPair<GetBlockHeaders>, GetBlockHeaders);
message_from_impl!(RequestPair<BlockHeaders>, BlockHeaders);
message_from_impl!(RequestPair<GetBlockBodies>, GetBlockBodies);
message_from_impl!(RequestPair<BlockBodies>, BlockBodies);
message_from_impl!(RequestPair<GetPooledTransactions>, GetPooledTransactions);
message_from_impl!(RequestPair<PooledTransactions>, PooledTransactions);
message_from_impl!(RequestPair<GetNodeData>, GetNodeData);
message_from_impl!(RequestPair<NodeData>, NodeData);
message_from_impl!(RequestPair<GetReceipts>, GetReceipts);
message_from_impl!(RequestPair<Receipts>, Receipts);
