use std::fmt::Debug;

use fastrlp::{Decodable, Encodable, RlpDecodable, RlpEncodable};

use crate::{
    blocks::{BlockBodies, BlockHeaders, GetBlockBodies},
    broadcast::{NewBlock, NewBlockHashes, NewPooledTransactionHashes, Transactions},
    GetBlockHeaders, GetNodeData, GetPooledTransactions, GetReceipts, NodeData, PooledTransactions,
    Receipts, Status,
};

// TODO: determine whats up with this enum variant size warning

/// Represents a message in the eth wire protocol, versions 65 and 66.
///
/// The ethereum wire protocol is a set of messages that are broadcasted to the network in two
/// styles:
///  * A request message sent by a peer (such as `GetPooledTransactions`), and an associated response message (such as `PooledTransactions`).
///  * A message that is broadcast to the network, without a corresponding request.
///
///  The newer eth/66 is an efficiency upgrade on top of eth/65, introducing a request id to correlate
///  request-response message pairs.
#[derive(Clone, Debug, PartialEq)]
pub enum EthMessage {
    // Status is required for the protocol handshake
    Status(Status),

    // The following messages are broadcast to the network
    NewBlockHashes(NewBlockHashes),
    NewBlock(NewBlock),
    Transactions(Transactions),
    NewPooledTransactionHashes(NewPooledTransactionHashes),

    // The following messages are request-response message pairs
    GetBlockHeaders(RequestPair<GetBlockHeaders>),
    BlockHeaders(RequestPair<BlockHeaders>),
    GetBlockBodies(RequestPair<GetBlockBodies>),
    BlockBodies(RequestPair<BlockBodies>),
    GetPooledTransactions(RequestPair<GetPooledTransactions>),
    PooledTransactions(RequestPair<PooledTransactions>),
    GetNodeData(RequestPair<GetNodeData>),
    NodeData(RequestPair<NodeData>),
    GetReceipts(RequestPair<GetReceipts>),
    Receipts(RequestPair<Receipts>),
}

/// This represents a network message which has a request id
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RequestPair<T> {
    /// id for the contained request or response message
    pub request_id: u64,

    /// the request or response message payload
    pub message: T,
}

/// Allows request messages with request ids to be serialized as RLP.
impl<T> Encodable for RequestPair<T>
where
    T: Encodable + Clone,
{
    fn length(&self) -> usize {
        self.request_id.length() + self.message.length()
    }

    fn encode(&self, out: &mut dyn fastrlp::BufMut) {

        #[derive(RlpEncodable)]
        struct Pair<Z: Encodable> {
            pub request_id: u64,
            pub message: Z,
        }

        let encodable_pair = Pair {
            request_id: self.request_id,
            message: self.message.clone(),
        };

        encodable_pair.encode(out);
    }
}

/// Allows request messages with request ids to be deserialized as RLP.
impl<T> Decodable for RequestPair<T>
where
    T: Decodable + Clone,
{
    fn decode(buf: &mut &[u8]) -> Result<Self, fastrlp::DecodeError> {
        #[derive(RlpDecodable)]
        struct Pair<Z: Decodable> {
            pub request_id: u64,
            pub message: Z,
        }

        let pair: Pair<T> = Pair::decode(buf)?;

        Ok(Self {
            request_id: pair.request_id,
            message: pair.message,
        })
    }
}

#[cfg(test)]
mod test {
    use crate::message::RequestPair;
    use fastrlp::{Decodable, Encodable};
    use hex_literal::hex;

    fn encode<T: Encodable>(value: T) -> Vec<u8> {
        let mut buf = vec![];
        value.encode(&mut buf);
        buf
    }

    #[test]
    fn request_pair_encode() {
        let request_pair = RequestPair {
            request_id: 1337,
            message: vec![5u8],
        };

        // c5: start of list (c0) + len(full_list) (length is <55 bytes)
        // 82: 0x80 + len(1337)
        // 05 39: 1337 (request_id)
        // === full_list ===
        // c1: start of list (c0) + len(list) (length is <55 bytes)
        // 05: 5 (message)
        let expected = hex!("c5820539c105");
        let got = encode(request_pair);
        assert_eq!(
            expected[..],
            got,
            "expected: {:X?}, got: {:X?}",
            expected,
            got,
        );
    }

    #[test]
    fn request_pair_decode() {
        let mut raw_pair = &hex!("c5820539c105")[..];

        let expected = RequestPair {
            request_id: 1337,
            message: vec![5u8],
        };

        let got = RequestPair::<Vec<u8>>::decode(&mut raw_pair).unwrap();
        assert_eq!(expected, got);
    }
}
