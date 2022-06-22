use anvil_core::eth::transaction::TypedTransaction;
use fastrlp::{RlpDecodable, RlpEncodable};

/// A list of transaction hashes that the peer would like transaction bodies for.
#[derive(Clone, Debug, PartialEq, Eq, RlpEncodable, RlpDecodable)]
pub struct GetPooledTransactions(Vec<[u8; 32]>);

/// The response to [GetPooledTransactions](crate::GetPooledTransactions), containing the
/// transaction bodies associated with the requested hashes.
///
/// This response may not contain all bodies requested, but the bodies should be in the same order
/// as the request's hashes. Hashes may be skipped, and the client should ensure that each body
/// corresponds to a requested hash. Hashes may need to be re-requested if the bodies are not
/// included in the response.
#[derive(Clone, Debug, PartialEq, Eq, RlpEncodable, RlpDecodable)]
pub struct PooledTransactions(Vec<TypedTransaction>);
