use ethers::types::Bytes;

/// A request for state tree nodes corresponding to the given hashes.
/// TODO: how long does it take for a node to determine that it doesn't have the requested hashes?
/// Are there any limits on the size of GetNodeData?
pub struct GetNodeData(Vec<[u8; 32]>);

// TODO: Bytes fastrlp encoding / decoding

/// The response to [GetNodeData](crate::GetNodeData), containing the state tree nodes or contract
/// bytecode corresponding to the requested hashes.
///
/// Not all nodes are guaranteed to be returned by the peer.
/// TODO: are response size limits implemented by all peers? since you could return an arbitrary
/// number of bytes for a single hash, and the user would not know until receiving the entire
/// payload.
/// Does this also mean that fast sync effectively caps the size of a node in the state tree (or
/// contract size) to the payload limit? If I were to have a very large node, would a node be able
/// to fast sync at all past that block?
///
/// Also - do can values be skipped? I assume so, similar to other hash requests
pub struct NodeData(Vec<Bytes>);
