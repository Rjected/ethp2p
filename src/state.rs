use ethers::types::Bytes;

/// A request for state tree nodes corresponding to the given hashes.
pub struct GetNodeData(Vec<[u8; 32]>);

// TODO: Bytes fastrlp encoding / decoding

/// The response to [GetNodeData](crate::GetNodeData), containing the state tree nodes or contract
/// bytecode corresponding to the requested hashes.
///
/// Not all nodes are guaranteed to be returned by the peer.
pub struct NodeData(Vec<Bytes>);
