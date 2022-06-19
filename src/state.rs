use fastrlp::{RlpDecodable, RlpEncodable};

/// A request for state tree nodes corresponding to the given hashes.
#[derive(Clone, Debug, PartialEq, Eq, RlpEncodable, RlpDecodable)]
pub struct GetNodeData(Vec<[u8; 32]>);

/// The response to [GetNodeData](crate::GetNodeData), containing the state tree nodes or contract
/// bytecode corresponding to the requested hashes.
///
/// Not all nodes are guaranteed to be returned by the peer.
#[derive(Clone, Debug, PartialEq, Eq, RlpEncodable, RlpDecodable)]
pub struct NodeData(Vec<bytes::Bytes>);
