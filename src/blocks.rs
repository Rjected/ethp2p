use anvil_core::eth::block::Header;
use ethers::types::{transaction::eip2718::TypedTransaction, Block, Signature};
use fastrlp::{Decodable, Encodable, RlpDecodable, RlpEncodable};

/// A request for a peer to return block headers starting at the requested block
/// TODO: better comment including limit / skip / reverse rules
#[derive(Copy, Clone, Debug, PartialEq, Eq, RlpEncodable, RlpDecodable)]
pub struct GetBlockHeaders {
    /// The block's number or hash that the peer should start returning headers from
    pub start_block: BlockHashOrNumber,

    /// The maximum number of headers to return
    ///
    /// TODO: should this be limited? does anything denote no limit? if so, this should be a more
    /// expressive type
    pub limit: u64,

    /// The number of blocks that the node should skip while traversing headers to return
    /// TODO: better comment
    pub skip: u32,

    /// Whether or not the headers should be returned in reverse order.
    pub reverse: bool,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
/// A Block Hash or Block Number
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
    fn decode(buf: &mut &[u8]) -> Result<Self, fastrlp::DecodeError> {
        let header: u8 = *buf.first().ok_or(fastrlp::DecodeError::InputTooShort)?;
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

// TODO: header fastrlp

/// The response to [GetBlockHeaders](crate::GetBlockHeaders), containing headers if any headers were
/// found.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BlockHeaders(Vec<Header>);

/// A request for a peer to return block bodies for the given block hashes.
#[derive(Clone, Debug, PartialEq, Eq, RlpEncodable, RlpDecodable)]
pub struct GetBlockBodies(Vec<[u8; 32]>);

// TODO: again, signed message in ethers
/// The response to [GetBlockBodies](crate::GetBlockBodies), containing the block bodies that the
/// peer knows about if any were found.
#[derive(Clone, Debug, PartialEq)]
pub struct BlockBodies(Vec<Block<(TypedTransaction, Signature)>>);

#[cfg(test)]
mod test {
    use fastrlp::Decodable;
    use hex_literal::hex;

    use super::BlockHashOrNumber;

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
}
