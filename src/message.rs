use std::fmt::Debug;

use open_fastrlp::{length_of_length, Decodable, Encodable, Header};

use crate::{
    blocks::{BlockBodies, BlockHeaders, GetBlockBodies},
    broadcast::{NewBlock, NewBlockHashes, NewPooledTransactionHashes, Transactions},
    GetBlockHeaders, GetNodeData, GetPooledTransactions, GetReceipts, NodeData, PooledTransactions,
    Receipts, Status,
};

#[derive(Clone, Debug, PartialEq, Eq)]
/// An `eth` protocol message, containing a message ID and payload.
pub struct ProtocolMessage {
    pub message_type: EthMessageID,
    pub message: EthMessage,
}

impl ProtocolMessage {
    /// Create a new ProtocolMessage from a message type and message rlp bytes.
    pub fn decode_message(
        message_type: EthMessageID,
        buf: &mut &[u8],
    ) -> Result<Self, open_fastrlp::DecodeError> {
        let message = match message_type {
            EthMessageID::Status => EthMessage::Status(Status::decode(buf)?),
            EthMessageID::NewBlockHashes => {
                EthMessage::NewBlockHashes(NewBlockHashes::decode(buf)?)
            }
            EthMessageID::NewBlock => EthMessage::NewBlock(Box::new(NewBlock::decode(buf)?)),
            EthMessageID::Transactions => EthMessage::Transactions(Transactions::decode(buf)?),
            EthMessageID::NewPooledTransactionHashes => {
                EthMessage::NewPooledTransactionHashes(NewPooledTransactionHashes::decode(buf)?)
            }
            EthMessageID::GetBlockHeaders => {
                let request_pair = RequestPair::<GetBlockHeaders>::decode(buf)?;
                EthMessage::GetBlockHeaders(request_pair)
            }
            EthMessageID::BlockHeaders => {
                let request_pair = RequestPair::<BlockHeaders>::decode(buf)?;
                EthMessage::BlockHeaders(request_pair)
            }
            EthMessageID::GetBlockBodies => {
                let request_pair = RequestPair::<GetBlockBodies>::decode(buf)?;
                EthMessage::GetBlockBodies(request_pair)
            }
            EthMessageID::BlockBodies => {
                let request_pair = RequestPair::<BlockBodies>::decode(buf)?;
                EthMessage::BlockBodies(request_pair)
            }
            EthMessageID::GetPooledTransactions => {
                let request_pair = RequestPair::<GetPooledTransactions>::decode(buf)?;
                EthMessage::GetPooledTransactions(request_pair)
            }
            EthMessageID::PooledTransactions => {
                let request_pair = RequestPair::<PooledTransactions>::decode(buf)?;
                EthMessage::PooledTransactions(request_pair)
            }
            EthMessageID::GetNodeData => {
                let request_pair = RequestPair::<GetNodeData>::decode(buf)?;
                EthMessage::GetNodeData(request_pair)
            }
            EthMessageID::NodeData => {
                let request_pair = RequestPair::<NodeData>::decode(buf)?;
                EthMessage::NodeData(request_pair)
            }
            EthMessageID::GetReceipts => {
                let request_pair = RequestPair::<GetReceipts>::decode(buf)?;
                EthMessage::GetReceipts(request_pair)
            }
            EthMessageID::Receipts => {
                let request_pair = RequestPair::<Receipts>::decode(buf)?;
                EthMessage::Receipts(request_pair)
            }
        };
        Ok(ProtocolMessage {
            message_type,
            message,
        })
    }
}

/// Encodes the protocol message into bytes.
/// The message type is encoded as a single byte and prepended to the message.
impl Encodable for ProtocolMessage {
    fn length(&self) -> usize {
        self.message_type.length() + self.message.length()
    }
    fn encode(&self, out: &mut dyn bytes::BufMut) {
        self.message_type.encode(out);
        self.message.encode(out);
    }
}

/// Decodes a protocol message from bytes, using the first byte to determine the message type.
/// This decodes `eth/66` request ids for each message type.
impl Decodable for ProtocolMessage {
    fn decode(buf: &mut &[u8]) -> Result<Self, open_fastrlp::DecodeError> {
        let message_type = EthMessageID::decode(buf)?;
        Self::decode_message(message_type, buf)
    }
}

impl From<EthMessage> for ProtocolMessage {
    fn from(message: EthMessage) -> Self {
        ProtocolMessage {
            message_type: message.message_id(),
            message,
        }
    }
}

// TODO: determine whats up with this enum variant size warning

/// Represents a message in the eth wire protocol, versions 66 and 67.
///
/// The ethereum wire protocol is a set of messages that are broadcasted to the network in two
/// styles:
///  * A request message sent by a peer (such as [`GetPooledTransactions`]), and an associated
///  response message (such as [`PooledTransactions`]).
///  * A message that is broadcast to the network, without a corresponding request.
///
///  The newer `eth/66` is an efficiency upgrade on top of `eth/65`, introducing a request id to
///  correlate request-response message pairs. This allows for request multiplexing.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum EthMessage {
    // Status is required for the protocol handshake
    Status(Status),

    // The following messages are broadcast to the network
    NewBlockHashes(NewBlockHashes),
    NewBlock(Box<NewBlock>),
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

impl EthMessage {
    /// Returns the message's ID.
    pub fn message_id(&self) -> EthMessageID {
        match self {
            EthMessage::Status(_) => EthMessageID::Status,
            EthMessage::NewBlockHashes(_) => EthMessageID::NewBlockHashes,
            EthMessage::NewBlock(_) => EthMessageID::NewBlock,
            EthMessage::Transactions(_) => EthMessageID::Transactions,
            EthMessage::NewPooledTransactionHashes(_) => EthMessageID::NewPooledTransactionHashes,
            EthMessage::GetBlockHeaders(_) => EthMessageID::GetBlockHeaders,
            EthMessage::BlockHeaders(_) => EthMessageID::BlockHeaders,
            EthMessage::GetBlockBodies(_) => EthMessageID::GetBlockBodies,
            EthMessage::BlockBodies(_) => EthMessageID::BlockBodies,
            EthMessage::GetPooledTransactions(_) => EthMessageID::GetPooledTransactions,
            EthMessage::PooledTransactions(_) => EthMessageID::PooledTransactions,
            EthMessage::GetNodeData(_) => EthMessageID::GetNodeData,
            EthMessage::NodeData(_) => EthMessageID::NodeData,
            EthMessage::GetReceipts(_) => EthMessageID::GetReceipts,
            EthMessage::Receipts(_) => EthMessageID::Receipts,
        }
    }
}

impl Encodable for EthMessage {
    fn length(&self) -> usize {
        match self {
            EthMessage::Status(status) => status.length(),
            EthMessage::NewBlockHashes(new_block_hashes) => new_block_hashes.length(),
            EthMessage::NewBlock(new_block) => new_block.length(),
            EthMessage::Transactions(transactions) => transactions.length(),
            EthMessage::NewPooledTransactionHashes(hashes) => hashes.length(),
            EthMessage::GetBlockHeaders(request) => request.length(),
            EthMessage::BlockHeaders(headers) => headers.length(),
            EthMessage::GetBlockBodies(request) => request.length(),
            EthMessage::BlockBodies(bodies) => bodies.length(),
            EthMessage::GetPooledTransactions(request) => request.length(),
            EthMessage::PooledTransactions(transactions) => transactions.length(),
            EthMessage::GetNodeData(request) => request.length(),
            EthMessage::NodeData(data) => data.length(),
            EthMessage::GetReceipts(request) => request.length(),
            EthMessage::Receipts(receipts) => receipts.length(),
        }
    }
    fn encode(&self, out: &mut dyn bytes::BufMut) {
        match self {
            EthMessage::Status(status) => status.encode(out),
            EthMessage::NewBlockHashes(new_block_hashes) => new_block_hashes.encode(out),
            EthMessage::NewBlock(new_block) => new_block.encode(out),
            EthMessage::Transactions(transactions) => transactions.encode(out),
            EthMessage::NewPooledTransactionHashes(hashes) => hashes.encode(out),
            EthMessage::GetBlockHeaders(request) => request.encode(out),
            EthMessage::BlockHeaders(headers) => headers.encode(out),
            EthMessage::GetBlockBodies(request) => request.encode(out),
            EthMessage::BlockBodies(bodies) => bodies.encode(out),
            EthMessage::GetPooledTransactions(request) => request.encode(out),
            EthMessage::PooledTransactions(transactions) => transactions.encode(out),
            EthMessage::GetNodeData(request) => request.encode(out),
            EthMessage::NodeData(data) => data.encode(out),
            EthMessage::GetReceipts(request) => request.encode(out),
            EthMessage::Receipts(receipts) => receipts.encode(out),
        }
    }
}

/// Represents message IDs for eth protocol messages.
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EthMessageID {
    Status = 0x00,
    NewBlockHashes = 0x01,
    Transactions = 0x02,
    GetBlockHeaders = 0x03,
    BlockHeaders = 0x04,
    GetBlockBodies = 0x05,
    BlockBodies = 0x06,
    NewBlock = 0x07,
    NewPooledTransactionHashes = 0x08,
    GetPooledTransactions = 0x09,
    PooledTransactions = 0x0a,
    GetNodeData = 0x0d,
    NodeData = 0x0e,
    GetReceipts = 0x0f,
    Receipts = 0x10,
}

impl Encodable for EthMessageID {
    fn length(&self) -> usize {
        1
    }
    fn encode(&self, out: &mut dyn bytes::BufMut) {
        out.put_u8(*self as u8);
    }
}

impl Decodable for EthMessageID {
    fn decode(buf: &mut &[u8]) -> Result<Self, open_fastrlp::DecodeError> {
        let id = buf
            .first()
            .ok_or(open_fastrlp::DecodeError::InputTooShort)?;
        Ok(match id {
            0x00 => EthMessageID::Status,
            0x01 => EthMessageID::NewBlockHashes,
            0x02 => EthMessageID::Transactions,
            0x03 => EthMessageID::GetBlockHeaders,
            0x04 => EthMessageID::BlockHeaders,
            0x05 => EthMessageID::GetBlockBodies,
            0x06 => EthMessageID::BlockBodies,
            0x07 => EthMessageID::NewBlock,
            0x08 => EthMessageID::NewPooledTransactionHashes,
            0x09 => EthMessageID::GetPooledTransactions,
            0x0a => EthMessageID::PooledTransactions,
            0x0d => EthMessageID::GetNodeData,
            0x0e => EthMessageID::NodeData,
            0x0f => EthMessageID::GetReceipts,
            0x10 => EthMessageID::Receipts,
            _ => return Err(open_fastrlp::DecodeError::Custom("Invalid message ID")),
        })
    }
}

impl TryFrom<usize> for EthMessageID {
    type Error = &'static str;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        match value {
            0x00 => Ok(EthMessageID::Status),
            0x01 => Ok(EthMessageID::NewBlockHashes),
            0x02 => Ok(EthMessageID::Transactions),
            0x03 => Ok(EthMessageID::GetBlockHeaders),
            0x04 => Ok(EthMessageID::BlockHeaders),
            0x05 => Ok(EthMessageID::GetBlockBodies),
            0x06 => Ok(EthMessageID::BlockBodies),
            0x07 => Ok(EthMessageID::NewBlock),
            0x08 => Ok(EthMessageID::NewPooledTransactionHashes),
            0x09 => Ok(EthMessageID::GetPooledTransactions),
            0x0a => Ok(EthMessageID::PooledTransactions),
            0x0d => Ok(EthMessageID::GetNodeData),
            0x0e => Ok(EthMessageID::NodeData),
            0x0f => Ok(EthMessageID::GetReceipts),
            0x10 => Ok(EthMessageID::Receipts),
            _ => Err("Invalid message ID"),
        }
    }
}

/// This is used for all request-response style `eth` protocol messages.
/// This can represent either a request or a response, since both include a message payload and
/// request id.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RequestPair<T> {
    /// id for the contained request or response message
    pub request_id: u64,

    /// the request or response message payload
    pub message: T,
}

/// Allows messages with request ids to be serialized into RLP bytes.
impl<T> Encodable for RequestPair<T>
where
    T: Encodable,
{
    fn length(&self) -> usize {
        let mut length = 0;
        length += self.request_id.length();
        length += self.message.length();
        length += length_of_length(length);
        length
    }

    fn encode(&self, out: &mut dyn open_fastrlp::BufMut) {
        let header = Header {
            list: true,
            payload_length: self.request_id.length() + self.message.length(),
        };

        header.encode(out);
        self.request_id.encode(out);
        self.message.encode(out);
    }
}

/// Allows messages with request ids to be deserialized into RLP bytes.
impl<T> Decodable for RequestPair<T>
where
    T: Decodable,
{
    fn decode(buf: &mut &[u8]) -> Result<Self, open_fastrlp::DecodeError> {
        let _header = Header::decode(buf)?;
        Ok(Self {
            request_id: u64::decode(buf)?,
            message: T::decode(buf)?,
        })
    }
}

#[cfg(test)]
mod test {
    use crate::message::RequestPair;
    use hex_literal::hex;
    use open_fastrlp::{Decodable, Encodable};

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
            "expected: {expected:X?}, got: {got:X?}",
        );
    }

    #[test]
    fn request_pair_decode() {
        let raw_pair = &hex!("c5820539c105")[..];

        let expected = RequestPair {
            request_id: 1337,
            message: vec![5u8],
        };

        let got = RequestPair::<Vec<u8>>::decode(&mut &*raw_pair).unwrap();
        assert_eq!(expected.length(), raw_pair.len());
        assert_eq!(expected, got);
    }
}
