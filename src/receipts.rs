use ethers::prelude::TransactionReceipt;

/// A request for transaction receipts from the given block hashes.
pub struct GetReceipts(Vec<[u8; 32]>);

// TODO: fastrlp encoding / decoding for receipt types

/// The response to [GetReceipts](crate::GetReceipts), containing receipt lists that correspond to
/// each block requested.
pub struct Receipts(Vec<Vec<TransactionReceipt>>);
