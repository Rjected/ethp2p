use anvil_core::eth::transaction::TypedTransaction;
use fastrlp::{RlpDecodableWrapper, RlpEncodableWrapper};

/// A list of transaction hashes that the peer would like transaction bodies for.
#[derive(Clone, Debug, PartialEq, Eq, RlpEncodableWrapper, RlpDecodableWrapper)]
pub struct GetPooledTransactions(pub Vec<[u8; 32]>);

/// The response to [GetPooledTransactions](crate::GetPooledTransactions), containing the
/// transaction bodies associated with the requested hashes.
///
/// This response may not contain all bodies requested, but the bodies should be in the same order
/// as the request's hashes. Hashes may be skipped, and the client should ensure that each body
/// corresponds to a requested hash. Hashes may need to be re-requested if the bodies are not
/// included in the response.
#[derive(Clone, Debug, PartialEq, Eq, RlpEncodableWrapper, RlpDecodableWrapper)]
pub struct PooledTransactions(pub Vec<TypedTransaction>);

#[cfg(test)]
mod test {
    use hex_literal::hex;

    use crate::{message::RequestPair, GetPooledTransactions};
    use fastrlp::{Decodable, Encodable};

    #[test]
    // Test vector from: https://eips.ethereum.org/EIPS/eip-2481
    fn encode_get_pooled_transactions() {
        let expected = hex!("f847820457f842a000000000000000000000000000000000000000000000000000000000deadc0dea000000000000000000000000000000000000000000000000000000000feedbeef");
        let mut data = vec![];
        let request = RequestPair::<GetPooledTransactions> {
            request_id: 1111,
            message: GetPooledTransactions(vec![
                hex!("00000000000000000000000000000000000000000000000000000000deadc0de"),
                hex!("00000000000000000000000000000000000000000000000000000000feedbeef"),
            ]),
        };
        request.encode(&mut data);
        assert_eq!(data, expected);
    }

    #[test]
    // Test vector from: https://eips.ethereum.org/EIPS/eip-2481
    fn decode_get_pooled_transactions() {
        let data = hex!("f847820457f842a000000000000000000000000000000000000000000000000000000000deadc0dea000000000000000000000000000000000000000000000000000000000feedbeef");
        let request = RequestPair::<GetPooledTransactions>::decode(&mut &data[..]).unwrap();
        assert_eq!(
            request,
            RequestPair::<GetPooledTransactions> {
                request_id: 1111,
                message: GetPooledTransactions(vec![
                    hex!("00000000000000000000000000000000000000000000000000000000deadc0de"),
                    hex!("00000000000000000000000000000000000000000000000000000000feedbeef"),
                ])
            }
        );
    }
}
