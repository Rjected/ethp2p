use ethers::prelude::TransactionReceipt;

/// A request for transaction receipts from the given block hashes.
pub struct GetReceipts(Vec<[u8; 32]>);

// TODO: fastrlp encoding / decoding for receipt types

/// The response to [GetReceipts](crate::GetReceipts), containing receipt lists that correspond to
/// each block requested.
///
/// TODO: question: each response must contain the complete list of receipts for each block, are
/// they allowed to be skipped? how much work before clients understand that the request
/// is invalid?
pub struct Receipts(Vec<Vec<TransactionReceipt>>);
