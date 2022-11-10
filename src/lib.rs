mod broadcast;
pub use broadcast::{
    BlockHashNumber, NewBlock, NewBlockHashes, NewPooledTransactionHashes, Transactions,
};

mod message;
pub use message::{EthMessage, EthMessageID, ProtocolMessage, RequestPair};

mod request;
pub use request::Request;

mod response;
pub use response::Response;

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

mod forkid;
pub use forkid::{ForkFilter, ForkHash, ForkId};

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

// do the same for each variant of Request and Response

// impl from for each variant of Request
macro_rules! request_from_impl {
    ($t:ty, $variant:ident) => {
        impl From<$t> for Request {
            fn from(t: $t) -> Self {
                Request::$variant(t)
            }
        }
    };
}

request_from_impl!(Status, Status);
request_from_impl!(NewBlockHashes, NewBlockHashes);
request_from_impl!(Box<NewBlock>, NewBlock);
request_from_impl!(Transactions, Transactions);
request_from_impl!(NewPooledTransactionHashes, NewPooledTransactionHashes);
request_from_impl!(RequestPair<GetBlockHeaders>, GetBlockHeaders);
request_from_impl!(RequestPair<GetBlockBodies>, GetBlockBodies);
request_from_impl!(RequestPair<GetPooledTransactions>, GetPooledTransactions);
request_from_impl!(RequestPair<GetNodeData>, GetNodeData);
request_from_impl!(RequestPair<GetReceipts>, GetReceipts);

// impl from for each variant of Response
macro_rules! response_from_impl {
    ($t:ty, $variant:ident) => {
        impl From<$t> for Response {
            fn from(t: $t) -> Self {
                Response::$variant(t)
            }
        }
    };
}

response_from_impl!(Status, Status);
response_from_impl!(RequestPair<BlockHeaders>, BlockHeaders);
response_from_impl!(RequestPair<BlockBodies>, BlockBodies);
response_from_impl!(RequestPair<PooledTransactions>, PooledTransactions);
response_from_impl!(RequestPair<NodeData>, NodeData);
response_from_impl!(RequestPair<Receipts>, Receipts);
