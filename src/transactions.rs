use ethers::types::{transaction::eip2718::TypedTransaction, Signature};

/// A list of transaction hashes that the peer would like transaction bodies for.
pub struct GetPooledTransactions(Vec<[u8; 32]>);

/// The response to [GetPooledTransactions](crate::GetPooledTransactions), containing the
/// transaction bodies associated with the requested hashes.
///
/// This response may not contain all bodies requested, but the bodies should be in the same order
/// as the request's hashes. Hashes may be skipped, and the client should ensure that each body
/// corresponds to a requested hash. Hashes may need to be re-requested if they
/// TODO: question: do nodes automatically re request? nodes can implement arbitrary response
/// limits on the response size, what happens if we respond with a single hash, in a long ish
/// amount of time? the mempool would be populated more slowly I guess.
pub struct PooledTransactions(Vec<(TypedTransaction, Signature)>);
