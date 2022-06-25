use anvil_core::eth::receipt::TypedReceipt;
use fastrlp::{RlpDecodable, RlpEncodable};

/// A request for transaction receipts from the given block hashes.
#[derive(Clone, Debug, PartialEq, Eq, RlpEncodable, RlpDecodable)]
pub struct GetReceipts(pub Vec<[u8; 32]>);

/// The response to [GetReceipts](crate::GetReceipts), containing receipt lists that correspond to
/// each block requested.
#[derive(Clone, Debug, PartialEq, Eq, RlpEncodable, RlpDecodable)]
pub struct Receipts(pub Vec<Vec<TypedReceipt>>);
