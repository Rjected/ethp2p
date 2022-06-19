use ethers::prelude::TransactionReceipt;
use fastrlp::{RlpDecodable, RlpEncodable};

/// A request for transaction receipts from the given block hashes.
#[derive(Clone, Debug, PartialEq, Eq, RlpEncodable, RlpDecodable)]
pub struct GetReceipts(Vec<[u8; 32]>);

// TODO: fastrlp encoding / decoding for receipt types

/// The response to [GetReceipts](crate::GetReceipts), containing receipt lists that correspond to
/// each block requested.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Receipts(Vec<Vec<TransactionReceipt>>);
