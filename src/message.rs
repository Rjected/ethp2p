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
#[derive(Debug, PartialEq)]
pub struct RequestPair<T> {
    /// id for the contained request or response message
    pub request_id: u64,

    /// the request or response message payload
    pub message: Option<T>,
}

/// Allows request messages with request ids to be serialized as RLP.
/// All request messages encode the payload as a list after encoding the request id.
impl<T> Encodable for RequestPair<T>
where
    T: Encodable,
{
    fn length(&self) -> usize {
        // the length will be
        let message_len = match &self.message {
            Some(message) => message.length(),
            // if there is no payload, it will be represented as an empty list, c0, which has length 1
            None => 1,
        };
        self.request_id.length() + message_len
    }

    fn encode(&self, out: &mut dyn fastrlp::BufMut) {
        // no payload, regardless of type, means we will encode an empty list.
        let message_vec = match &self.message {
            Some(message) => vec![message],
            None => vec![],
        };

        #[derive(RlpEncodable)]
        struct Pair<Z: Encodable> {
            pub request_id: u64,
            pub message: Vec<Z>,
        }

        let encodable_pair = Pair {
            request_id: self.request_id,
            message: message_vec,
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
            pub message: Vec<Z>,
        }

        let pair: Pair<T> = Pair::decode(buf)?;
        if pair.message.len() > 1 {
            // TODO: better error message
            return Err(fastrlp::DecodeError::Custom(
                "Did not expect multiple requests while decoding",
            ));
        }

        // now we know there is a single element in the vec
        Ok(Self {
            request_id: pair.request_id,
            message: pair.message.first().map(|item| item.to_owned()),
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
            message: Some(5u8),
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
    fn empty_pair_encode() {
        let request_pair = RequestPair::<u8> {
            request_id: 1337,
            message: None,
        };

        // c4: start of list (c0) + len(full_list)
        // 82: 0x80 + len(1337)
        // 05 39: 1337 (request_id)
        // === full_list ===
        // c0: start of list (c0) + len(list) (length is 0)
        let expected = hex!("c4820539c0");
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
    fn empty_pair_decode() {
        let mut raw_pair = &hex!("c4820539c0")[..];

        let expected = RequestPair::<u8> {
            request_id: 1337,
            message: None,
        };
        let got = RequestPair::<u8>::decode(&mut raw_pair).unwrap();
        assert_eq!(expected, got);
    }

    #[test]
    fn request_pair_decode() {
        let mut raw_pair = &hex!("c5820539c105")[..];

        let expected = RequestPair {
            request_id: 1337,
            message: Some(5u8),
        };
        let got = RequestPair::<u8>::decode(&mut raw_pair).unwrap();
        assert_eq!(expected, got);
    }
}
