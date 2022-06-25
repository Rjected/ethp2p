use fastrlp::{RlpDecodableWrapper, RlpEncodableWrapper};

/// A request for state tree nodes corresponding to the given hashes.
#[derive(Clone, Debug, PartialEq, Eq, RlpEncodableWrapper, RlpDecodableWrapper)]
pub struct GetNodeData(pub Vec<[u8; 32]>);

/// The response to [GetNodeData](crate::GetNodeData), containing the state tree nodes or contract
/// bytecode corresponding to the requested hashes.
///
/// Not all nodes are guaranteed to be returned by the peer.
#[derive(Clone, Debug, PartialEq, Eq, RlpEncodableWrapper, RlpDecodableWrapper)]
pub struct NodeData(pub Vec<bytes::Bytes>);

#[cfg(test)]
mod test {
    use hex_literal::hex;

    use crate::{message::RequestPair, GetNodeData};
    use fastrlp::{Decodable, Encodable};

    #[test]
    // Test vector from: https://eips.ethereum.org/EIPS/eip-2481
    fn encode_get_node_data() {
        let expected = hex!("f847820457f842a000000000000000000000000000000000000000000000000000000000deadc0dea000000000000000000000000000000000000000000000000000000000feedbeef");
        let mut data = vec![];
        let request = RequestPair::<GetNodeData> {
            request_id: 1111,
            message: GetNodeData(vec![
                hex!("00000000000000000000000000000000000000000000000000000000deadc0de"),
                hex!("00000000000000000000000000000000000000000000000000000000feedbeef"),
            ]),
        };
        request.encode(&mut data);
        assert_eq!(data, expected);
    }

    #[test]
    // Test vector from: https://eips.ethereum.org/EIPS/eip-2481
    fn decode_get_node_data() {
        let data = hex!("f847820457f842a000000000000000000000000000000000000000000000000000000000deadc0dea000000000000000000000000000000000000000000000000000000000feedbeef");
        let request = RequestPair::<GetNodeData>::decode(&mut &data[..]).unwrap();
        assert_eq!(
            request,
            RequestPair::<GetNodeData> {
                request_id: 1111,
                message: GetNodeData(vec![
                    hex!("00000000000000000000000000000000000000000000000000000000deadc0de"),
                    hex!("00000000000000000000000000000000000000000000000000000000feedbeef"),
                ])
            }
        );
    }
}
