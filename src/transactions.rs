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
    use std::str::FromStr;

    use anvil_core::eth::transaction::{TypedTransaction, LegacyTransaction, TransactionKind};
    use ethers::prelude::{Bytes, Signature, U256};
    use hex_literal::hex;

    use crate::{message::RequestPair, GetPooledTransactions, PooledTransactions};
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

    #[test]
    // Test vector from: https://eips.ethereum.org/EIPS/eip-2481
    fn encode_pooled_transactions() {
        let expected = hex!("f8d7820457f8d2f867088504a817c8088302e2489435353535353535353535353535353535353535358202008025a064b1702d9298fee62dfeccc57d322a463ad55ca201256d01f62b45b2e1c21c12a064b1702d9298fee62dfeccc57d322a463ad55ca201256d01f62b45b2e1c21c10f867098504a817c809830334509435353535353535353535353535353535353535358202d98025a052f8f61201b2b11a78d6e866abc9c3db2ae8631fa656bfe5cb53668255367afba052f8f61201b2b11a78d6e866abc9c3db2ae8631fa656bfe5cb53668255367afb");
        let mut data = vec![];
        let request = RequestPair::<PooledTransactions> {
            request_id: 1111,
            message: PooledTransactions(vec![
                TypedTransaction::Legacy(LegacyTransaction {
                    nonce: 0x8u64.into(),
                    gas_price: 0x4a817c808u64.into(),
                    gas_limit: 0x2e248u64.into(),
                    kind: TransactionKind::Call(hex!("3535353535353535353535353535353535353535").into()),
                    value: 0x200u64.into(),
                    input: Bytes::default(),
                    signature: Signature {
                        v: 0x25u64,
                        r: U256::from_str("64b1702d9298fee62dfeccc57d322a463ad55ca201256d01f62b45b2e1c21c12").unwrap(),
                        s: U256::from_str("64b1702d9298fee62dfeccc57d322a463ad55ca201256d01f62b45b2e1c21c10").unwrap(),
                    }
                }),
                TypedTransaction::Legacy(LegacyTransaction {
                    nonce: 0x09u64.into(),
                    gas_price: 0x4a817c809u64.into(),
                    gas_limit: 0x33450u64.into(),
                    kind: TransactionKind::Call(hex!("3535353535353535353535353535353535353535").into()),
                    value: 0x2d9u64.into(),
                    input: Bytes::default(),
                    signature: Signature {
                        v: 0x25u64,
                        r: U256::from_str("52f8f61201b2b11a78d6e866abc9c3db2ae8631fa656bfe5cb53668255367afb").unwrap(),
                        s: U256::from_str("52f8f61201b2b11a78d6e866abc9c3db2ae8631fa656bfe5cb53668255367afb").unwrap(),
                    },
                }),
            ]),
        };
        request.encode(&mut data);
        assert_eq!(data, expected);
    }

    #[test]
    // Test vector from: https://eips.ethereum.org/EIPS/eip-2481
    fn decode_pooled_transactions() {
        let data = hex!("f8d7820457f8d2f867088504a817c8088302e2489435353535353535353535353535353535353535358202008025a064b1702d9298fee62dfeccc57d322a463ad55ca201256d01f62b45b2e1c21c12a064b1702d9298fee62dfeccc57d322a463ad55ca201256d01f62b45b2e1c21c10f867098504a817c809830334509435353535353535353535353535353535353535358202d98025a052f8f61201b2b11a78d6e866abc9c3db2ae8631fa656bfe5cb53668255367afba052f8f61201b2b11a78d6e866abc9c3db2ae8631fa656bfe5cb53668255367afb");
        let expected = RequestPair::<PooledTransactions> {
            request_id: 1111,
            message: PooledTransactions(vec![
                TypedTransaction::Legacy(LegacyTransaction {
                    nonce: 0x8u64.into(),
                    gas_price: 0x4a817c808u64.into(),
                    gas_limit: 0x2e248u64.into(),
                    kind: TransactionKind::Call(hex!("3535353535353535353535353535353535353535").into()),
                    value: 0x200u64.into(),
                    input: Bytes::default(),
                    signature: Signature {
                        v: 0x25u64,
                        r: U256::from_str("64b1702d9298fee62dfeccc57d322a463ad55ca201256d01f62b45b2e1c21c12").unwrap(),
                        s: U256::from_str("64b1702d9298fee62dfeccc57d322a463ad55ca201256d01f62b45b2e1c21c10").unwrap(),
                    }
                }),
                TypedTransaction::Legacy(LegacyTransaction {
                    nonce: 0x09u64.into(),
                    gas_price: 0x4a817c809u64.into(),
                    gas_limit: 0x33450u64.into(),
                    kind: TransactionKind::Call(hex!("3535353535353535353535353535353535353535").into()),
                    value: 0x2d9u64.into(),
                    input: Bytes::default(),
                    signature: Signature {
                        v: 0x25u64,
                        r: U256::from_str("52f8f61201b2b11a78d6e866abc9c3db2ae8631fa656bfe5cb53668255367afb").unwrap(),
                        s: U256::from_str("52f8f61201b2b11a78d6e866abc9c3db2ae8631fa656bfe5cb53668255367afb").unwrap(),
                    },
                }),
            ]),
        };

        let request = RequestPair::<PooledTransactions>::decode(&mut &data[..]).unwrap();
        assert_eq!(request, expected);
    }
}
