use anvil_core::eth::{
    block::{Block, Header},
    transaction::TypedTransaction,
};
use open_fastrlp::{
    Decodable, Encodable, RlpDecodable, RlpDecodableWrapper, RlpEncodable, RlpEncodableWrapper,
};
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
/// Either a block hash _or_ a block number
pub enum BlockHashOrNumber {
    /// A block hash
    Hash([u8; 32]),
    /// A block number
    Number(u64),
}

/// Allows for RLP encoding of either a block hash or block number
impl Encodable for BlockHashOrNumber {
    fn length(&self) -> usize {
        match self {
            Self::Hash(block_hash) => block_hash.length(),
            Self::Number(block_number) => block_number.length(),
        }
    }
    fn encode(&self, out: &mut dyn bytes::BufMut) {
        match self {
            Self::Hash(block_hash) => block_hash.encode(out),
            Self::Number(block_number) => block_number.encode(out),
        }
    }
}

/// Allows for RLP decoding of a block hash or block number
impl Decodable for BlockHashOrNumber {
    fn decode(buf: &mut &[u8]) -> Result<Self, open_fastrlp::DecodeError> {
        let header: u8 = *buf.first().ok_or(open_fastrlp::DecodeError::InputTooShort)?;
        // if the byte string is exactly 32 bytes, decode it into a Hash
        // 0xa0 = 0x80 (start of string) + 0x20 (32, length of string)
        if header == 0xa0 {
            // strip the first byte, parsing the rest of the string.
            // If the rest of the string fails to decode into 32 bytes, we'll bubble up the
            // decoding error.
            let hash = <[u8; 32]>::decode(buf)?;
            Ok(Self::Hash(hash))
        } else {
            // a block number when encoded as bytes ranges from 0 to any number of bytes - we're
            // going to accept numbers which fit in less than 64 bytes.
            // Any data larger than this which is not caught by the Hash decoding should error and
            // is considered an invalid block number.
            Ok(Self::Number(<u64>::decode(buf)?))
        }
    }
}

/// A request for a peer to return block headers starting at the requested block.
/// The peer must return at most [`limit`](#structfield.limit) headers.
/// If the [`reverse`](#structfield.reverse) field is `true`, the headers will be returned starting
/// at [`start_block`](#structfield.start_block), traversing towards the genesis block.
/// Otherwise, headers will be returned starting at [`start_block`](#structfield.start_block),
/// traversing towards the latest block.
///
/// If the [`skip`](#structfield.skip) field is non-zero, the peer must skip that amount of headers
/// in the the direction specified by [`reverse`](#structfield.reverse).
#[derive(Copy, Clone, Debug, PartialEq, Eq, RlpEncodable, RlpDecodable)]
pub struct GetBlockHeaders {
    /// The block number or hash that the peer should start returning headers from.
    pub start_block: BlockHashOrNumber,

    /// The maximum number of headers to return.
    pub limit: u64,

    /// The number of blocks that the node should skip while traversing and returning headers.
    /// A skip value of zero denotes that the peer should return contiguous heaaders, starting from
    /// [`start_block`](#structfield.start_block) and returning at most [`limit`](#structfield.limit)
    /// headers.
    pub skip: u32,

    /// Whether or not the headers should be returned in reverse order.
    pub reverse: bool,
}

/// The response to [`GetBlockHeaders`], containing headers if any headers were found.
#[derive(Clone, Debug, PartialEq, Eq, RlpEncodableWrapper, RlpDecodableWrapper)]
pub struct BlockHeaders(
    /// The requested headers.
    pub Vec<Header>,
);

impl From<Vec<Header>> for BlockHeaders {
    fn from(headers: Vec<Header>) -> Self {
        BlockHeaders(headers)
    }
}

/// A request for a peer to return block bodies for the given block hashes.
#[derive(Clone, Debug, PartialEq, Eq, RlpEncodableWrapper, RlpDecodableWrapper)]
pub struct GetBlockBodies(
    /// The block hashes to request bodies for.
    pub Vec<[u8; 32]>,
);

impl From<Vec<[u8; 32]>> for GetBlockBodies {
    fn from(hashes: Vec<[u8; 32]>) -> Self {
        GetBlockBodies(hashes)
    }
}

/// A response to [`GetBlockBodies`], containing bodies if any bodies were found.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, RlpEncodable, RlpDecodable)]
pub struct BlockBody {
    pub transactions: Vec<TypedTransaction>,
    pub ommers: Vec<Header>,
}

impl BlockBody {
    /// Create a [`Block`] from the body and its header.
    pub fn create_block(&self, header: &Header) -> Block {
        Block {
            header: header.clone(),
            transactions: self.transactions.clone(),
            ommers: self.ommers.clone(),
        }
    }
}

/// The response to [`GetBlockBodies`], containing the block bodies that the peer knows about if
/// any were found.
#[derive(Clone, Debug, PartialEq, Eq, RlpEncodableWrapper, RlpDecodableWrapper)]
pub struct BlockBodies(
    /// The requested block bodies, each of which should correspond to a hash in the request.
    pub Vec<BlockBody>,
);

impl From<Vec<BlockBody>> for BlockBodies {
    fn from(bodies: Vec<BlockBody>) -> Self {
        BlockBodies(bodies)
    }
}

#[cfg(test)]
mod test {
    use std::str::FromStr;

    use anvil_core::eth::{
        block::Header,
        transaction::{LegacyTransaction, TransactionKind, TypedTransaction},
    };
    use ethers::core::types::{Bytes, Signature, H64, U256};
    use open_fastrlp::{Decodable, Encodable};
    use hex_literal::hex;

    use crate::{message::RequestPair, BlockBodies, BlockHeaders, GetBlockBodies, GetBlockHeaders};

    use super::{BlockBody, BlockHashOrNumber};

    #[test]
    fn decode_hash() {
        // this is a valid 32 byte rlp string
        let rlp = hex!("a0ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff");
        let decoded_number = BlockHashOrNumber::decode(&mut &rlp[..]).unwrap();
        let full_bytes = [0xff; 32];
        let expected = BlockHashOrNumber::Hash(full_bytes);
        assert_eq!(expected, decoded_number);
    }

    #[test]
    fn decode_number() {
        // this is a valid 64 bit number
        let rlp = hex!("88ffffffffffffffff");
        let decoded_number = BlockHashOrNumber::decode(&mut &rlp[..]).unwrap();
        let expected = BlockHashOrNumber::Number(u64::MAX);
        assert_eq!(expected, decoded_number);
    }

    #[test]
    fn decode_largest_single_byte() {
        // the largest single byte is 0x7f, so we should be able to decode this into a u64
        let rlp = hex!("7f");
        let decoded_number = BlockHashOrNumber::decode(&mut &rlp[..]).unwrap();
        let expected = BlockHashOrNumber::Number(0x7fu64);
        assert_eq!(expected, decoded_number);
    }

    #[test]
    fn decode_long_hash() {
        // let's try a 33 byte long string
        // 0xa1 = 0x80 (start of string) + 0x21 (33, length of string)
        let long_rlp = hex!("a1ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff");
        let decode_result = BlockHashOrNumber::decode(&mut &long_rlp[..]);
        if decode_result.is_ok() {
            panic!("Decoding a bytestring longer than 32 bytes should not decode successfully");
        }
    }

    #[test]
    fn decode_long_number() {
        // let's try a 72 bit number
        // 0x89 = 0x80 (start of string) + 0x09 (9, length of string)
        let long_number = hex!("89ffffffffffffffffff");
        let decode_result = BlockHashOrNumber::decode(&mut &long_number[..]);
        if decode_result.is_ok() {
            panic!("Decoding a number longer than 64 bits (but not exactly 32 bytes) should not decode successfully");
        }
    }

    #[test]
    // Test vector from: https://eips.ethereum.org/EIPS/eip-2481
    fn encode_get_block_header() {
        let expected = hex!(
            "e8820457e4a000000000000000000000000000000000000000000000000000000000deadc0de050580"
        );
        let mut data = vec![];
        RequestPair::<GetBlockHeaders> {
            request_id: 1111,
            message: GetBlockHeaders {
                start_block: BlockHashOrNumber::Hash(hex!(
                    "00000000000000000000000000000000000000000000000000000000deadc0de"
                )),
                limit: 5,
                skip: 5,
                reverse: false,
            },
        }
        .encode(&mut data);
        assert_eq!(data, expected);
    }

    #[test]
    // Test vector from: https://eips.ethereum.org/EIPS/eip-2481
    fn decode_get_block_header() {
        let data = hex!(
            "e8820457e4a000000000000000000000000000000000000000000000000000000000deadc0de050580"
        );
        let expected = RequestPair::<GetBlockHeaders> {
            request_id: 1111,
            message: GetBlockHeaders {
                start_block: BlockHashOrNumber::Hash(hex!(
                    "00000000000000000000000000000000000000000000000000000000deadc0de"
                )),
                limit: 5,
                skip: 5,
                reverse: false,
            },
        };
        let result = RequestPair::decode(&mut &data[..]);
        assert_eq!(result.unwrap(), expected);
    }

    #[test]
    // Test vector from: https://eips.ethereum.org/EIPS/eip-2481
    fn encode_get_block_header_number() {
        let expected = hex!("ca820457c682270f050580");
        let mut data = vec![];
        RequestPair::<GetBlockHeaders> {
            request_id: 1111,
            message: GetBlockHeaders {
                start_block: BlockHashOrNumber::Number(9999),
                limit: 5,
                skip: 5,
                reverse: false,
            },
        }
        .encode(&mut data);
        assert_eq!(data, expected);
    }

    #[test]
    // Test vector from: https://eips.ethereum.org/EIPS/eip-2481
    fn decode_get_block_header_number() {
        let data = hex!("ca820457c682270f050580");
        let expected = RequestPair::<GetBlockHeaders> {
            request_id: 1111,
            message: GetBlockHeaders {
                start_block: BlockHashOrNumber::Number(9999),
                limit: 5,
                skip: 5,
                reverse: false,
            },
        };
        let result = RequestPair::decode(&mut &data[..]);
        assert_eq!(result.unwrap(), expected);
    }

    #[test]
    // Test vector from: https://eips.ethereum.org/EIPS/eip-2481
    fn encode_block_header() {
        // [ (f90202) 0x0457 = 1111, [ (f901fc) [ (f901f9) header ] ] ]
        let expected = hex!("f90202820457f901fcf901f9a00000000000000000000000000000000000000000000000000000000000000000a00000000000000000000000000000000000000000000000000000000000000000940000000000000000000000000000000000000000a00000000000000000000000000000000000000000000000000000000000000000a00000000000000000000000000000000000000000000000000000000000000000a00000000000000000000000000000000000000000000000000000000000000000b90100000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000008208ae820d0582115c8215b3821a0a827788a00000000000000000000000000000000000000000000000000000000000000000880000000000000000");
        let mut data = vec![];
        RequestPair::<BlockHeaders> {
            request_id: 1111,
            message: BlockHeaders(vec![
                Header {
                    parent_hash: hex!("0000000000000000000000000000000000000000000000000000000000000000").into(),
                    ommers_hash: hex!("0000000000000000000000000000000000000000000000000000000000000000").into(),
                    beneficiary: hex!("0000000000000000000000000000000000000000").into(),
                    state_root: hex!("0000000000000000000000000000000000000000000000000000000000000000").into(),
                    transactions_root: hex!("0000000000000000000000000000000000000000000000000000000000000000").into(),
                    receipts_root: hex!("0000000000000000000000000000000000000000000000000000000000000000").into(),
                    logs_bloom: hex!("00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000").into(),
                    difficulty: 0x8aeu64.into(),
                    number: 0xd05u64.into(),
                    gas_limit: 0x115cu64.into(),
                    gas_used: 0x15b3u64.into(),
                    timestamp: 0x1a0au64,
                    extra_data: hex!("7788").into(),
                    mix_hash: hex!("0000000000000000000000000000000000000000000000000000000000000000").into(),
                    nonce: H64::from_low_u64_be(0x0000000000000000u64),
                    base_fee_per_gas: None,
                },
            ]),
        }.encode(&mut data);
        assert_eq!(data, expected);
    }

    #[test]
    // Test vector from: https://eips.ethereum.org/EIPS/eip-2481
    fn decode_block_header() {
        let data = hex!("f90202820457f901fcf901f9a00000000000000000000000000000000000000000000000000000000000000000a00000000000000000000000000000000000000000000000000000000000000000940000000000000000000000000000000000000000a00000000000000000000000000000000000000000000000000000000000000000a00000000000000000000000000000000000000000000000000000000000000000a00000000000000000000000000000000000000000000000000000000000000000b90100000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000008208ae820d0582115c8215b3821a0a827788a00000000000000000000000000000000000000000000000000000000000000000880000000000000000");
        let expected = RequestPair::<BlockHeaders> {
            request_id: 1111,
            message: BlockHeaders(vec![
                Header {
                    parent_hash: hex!("0000000000000000000000000000000000000000000000000000000000000000").into(),
                    ommers_hash: hex!("0000000000000000000000000000000000000000000000000000000000000000").into(),
                    beneficiary: hex!("0000000000000000000000000000000000000000").into(),
                    state_root: hex!("0000000000000000000000000000000000000000000000000000000000000000").into(),
                    transactions_root: hex!("0000000000000000000000000000000000000000000000000000000000000000").into(),
                    receipts_root: hex!("0000000000000000000000000000000000000000000000000000000000000000").into(),
                    logs_bloom: hex!("00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000").into(),
                    difficulty: 0x8aeu64.into(),
                    number: 0xd05u64.into(),
                    gas_limit: 0x115cu64.into(),
                    gas_used: 0x15b3u64.into(),
                    timestamp: 0x1a0au64,
                    extra_data: hex!("7788").into(),
                    mix_hash: hex!("0000000000000000000000000000000000000000000000000000000000000000").into(),
                    nonce: H64::from_low_u64_be(0x0000000000000000u64),
                    base_fee_per_gas: None,
                },
            ]),
        };
        let result = RequestPair::decode(&mut &data[..]);
        assert_eq!(result.unwrap(), expected);
    }

    #[test]
    // Test vector from: https://eips.ethereum.org/EIPS/eip-2481
    fn encode_get_block_bodies() {
        let expected = hex!("f847820457f842a000000000000000000000000000000000000000000000000000000000deadc0dea000000000000000000000000000000000000000000000000000000000feedbeef");
        let mut data = vec![];
        RequestPair::<GetBlockBodies> {
            request_id: 1111,
            message: GetBlockBodies(vec![
                hex!("00000000000000000000000000000000000000000000000000000000deadc0de"),
                hex!("00000000000000000000000000000000000000000000000000000000feedbeef"),
            ]),
        }
        .encode(&mut data);
        assert_eq!(data, expected);
    }

    #[test]
    // Test vector from: https://eips.ethereum.org/EIPS/eip-2481
    fn decode_get_block_bodies() {
        let data = hex!("f847820457f842a000000000000000000000000000000000000000000000000000000000deadc0dea000000000000000000000000000000000000000000000000000000000feedbeef");
        let expected = RequestPair::<GetBlockBodies> {
            request_id: 1111,
            message: GetBlockBodies(vec![
                hex!("00000000000000000000000000000000000000000000000000000000deadc0de"),
                hex!("00000000000000000000000000000000000000000000000000000000feedbeef"),
            ]),
        };
        let result = RequestPair::decode(&mut &data[..]);
        assert_eq!(result.unwrap(), expected);
    }

    #[test]
    // Test vector from: https://eips.ethereum.org/EIPS/eip-2481
    fn encode_block_bodies() {
        let expected = hex!("f902dc820457f902d6f902d3f8d2f867088504a817c8088302e2489435353535353535353535353535353535353535358202008025a064b1702d9298fee62dfeccc57d322a463ad55ca201256d01f62b45b2e1c21c12a064b1702d9298fee62dfeccc57d322a463ad55ca201256d01f62b45b2e1c21c10f867098504a817c809830334509435353535353535353535353535353535353535358202d98025a052f8f61201b2b11a78d6e866abc9c3db2ae8631fa656bfe5cb53668255367afba052f8f61201b2b11a78d6e866abc9c3db2ae8631fa656bfe5cb53668255367afbf901fcf901f9a00000000000000000000000000000000000000000000000000000000000000000a00000000000000000000000000000000000000000000000000000000000000000940000000000000000000000000000000000000000a00000000000000000000000000000000000000000000000000000000000000000a00000000000000000000000000000000000000000000000000000000000000000a00000000000000000000000000000000000000000000000000000000000000000b90100000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000008208ae820d0582115c8215b3821a0a827788a00000000000000000000000000000000000000000000000000000000000000000880000000000000000");
        let mut data = vec![];
        let request = RequestPair::<BlockBodies> {
            request_id: 1111,
            message: BlockBodies(vec![
                BlockBody {
                    transactions: vec![
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
                            nonce: 0x9u64.into(),
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
                    ],
                    ommers: vec![
                        Header {
                            parent_hash: hex!("0000000000000000000000000000000000000000000000000000000000000000").into(),
                            ommers_hash: hex!("0000000000000000000000000000000000000000000000000000000000000000").into(),
                            beneficiary: hex!("0000000000000000000000000000000000000000").into(),
                            state_root: hex!("0000000000000000000000000000000000000000000000000000000000000000").into(),
                            transactions_root: hex!("0000000000000000000000000000000000000000000000000000000000000000").into(),
                            receipts_root: hex!("0000000000000000000000000000000000000000000000000000000000000000").into(),
                            logs_bloom: hex!("00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000").into(),
                            difficulty: 0x8aeu64.into(),
                            number: 0xd05u64.into(),
                            gas_limit: 0x115cu64.into(),
                            gas_used: 0x15b3u64.into(),
                            timestamp: 0x1a0au64,
                            extra_data: hex!("7788").into(),
                            mix_hash: hex!("0000000000000000000000000000000000000000000000000000000000000000").into(),
                            nonce: H64::from_low_u64_be(0x0000000000000000u64),
                            base_fee_per_gas: None,
                        },
                    ],
                }
            ]),
        };
        request.encode(&mut data);
        assert_eq!(data, expected);
    }

    #[test]
    // Test vector from: https://eips.ethereum.org/EIPS/eip-2481
    fn decode_block_bodies() {
        let data = hex!("f902dc820457f902d6f902d3f8d2f867088504a817c8088302e2489435353535353535353535353535353535353535358202008025a064b1702d9298fee62dfeccc57d322a463ad55ca201256d01f62b45b2e1c21c12a064b1702d9298fee62dfeccc57d322a463ad55ca201256d01f62b45b2e1c21c10f867098504a817c809830334509435353535353535353535353535353535353535358202d98025a052f8f61201b2b11a78d6e866abc9c3db2ae8631fa656bfe5cb53668255367afba052f8f61201b2b11a78d6e866abc9c3db2ae8631fa656bfe5cb53668255367afbf901fcf901f9a00000000000000000000000000000000000000000000000000000000000000000a00000000000000000000000000000000000000000000000000000000000000000940000000000000000000000000000000000000000a00000000000000000000000000000000000000000000000000000000000000000a00000000000000000000000000000000000000000000000000000000000000000a00000000000000000000000000000000000000000000000000000000000000000b90100000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000008208ae820d0582115c8215b3821a0a827788a00000000000000000000000000000000000000000000000000000000000000000880000000000000000");
        let expected = RequestPair::<BlockBodies> {
            request_id: 1111,
            message: BlockBodies(vec![
                BlockBody {
                    transactions: vec![
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
                            nonce: 0x9u64.into(),
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
                    ],
                    ommers: vec![
                        Header {
                            parent_hash: hex!("0000000000000000000000000000000000000000000000000000000000000000").into(),
                            ommers_hash: hex!("0000000000000000000000000000000000000000000000000000000000000000").into(),
                            beneficiary: hex!("0000000000000000000000000000000000000000").into(),
                            state_root: hex!("0000000000000000000000000000000000000000000000000000000000000000").into(),
                            transactions_root: hex!("0000000000000000000000000000000000000000000000000000000000000000").into(),
                            receipts_root: hex!("0000000000000000000000000000000000000000000000000000000000000000").into(),
                            logs_bloom: hex!("00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000").into(),
                            difficulty: 0x8aeu64.into(),
                            number: 0xd05u64.into(),
                            gas_limit: 0x115cu64.into(),
                            gas_used: 0x15b3u64.into(),
                            timestamp: 0x1a0au64,
                            extra_data: hex!("7788").into(),
                            mix_hash: hex!("0000000000000000000000000000000000000000000000000000000000000000").into(),
                            nonce: H64::from_low_u64_be(0x0000000000000000u64),
                            base_fee_per_gas: None,
                        },
                    ],
                }
            ]),
        };
        let result = RequestPair::decode(&mut &data[..]).unwrap();
        assert_eq!(result, expected);
    }
}
