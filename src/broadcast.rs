use anvil_core::eth::{block::Block, transaction::TypedTransaction};
use fastrlp::{RlpDecodable, RlpDecodableWrapper, RlpEncodable, RlpEncodableWrapper, length_of_length, Header, Decodable, Encodable};
use ruint::Uint;

/// This informs peers of new blocks that have appeared on the network.
#[derive(Clone, Debug, PartialEq, Eq, RlpEncodableWrapper, RlpDecodableWrapper)]
pub struct NewBlockHashes(pub Vec<BlockHashNumber>);

/// A block hash and a block number.
#[derive(Clone, Debug, PartialEq, Eq, RlpEncodable, RlpDecodable)]
pub struct BlockHashNumber {
    pub hash: [u8; 32],
    pub number: u64,
}

/// A new block with the current total difficultt, which includes the difficulty of the returned
/// block.
#[derive(Clone, Debug, PartialEq, Eq, RlpEncodable, RlpDecodable)]
pub struct NewBlock {
    pub block: Block,
    pub td: Uint<128, 2>,
}

// TODO: Introduce TypedTransaction signed message type (with fastrlp encoding) to ethers
/// This informs peers of transactions that have appeared on the network
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Transactions(pub Vec<TypedTransaction>);

impl Transactions {
    pub(crate) fn transactions_payload_length(&self) -> usize {
        let mut length = 0;
        for tx_response in &self.0 {
            length += match tx_response {
                TypedTransaction::Legacy(tx) => tx.length(),
                TypedTransaction::EIP2930(tx) => {
                    length_of_length(tx.length()) + tx.length() + 1
                }
                TypedTransaction::EIP1559(tx) => {
                    length_of_length(tx.length()) + tx.length() + 1
                }
            }
        }
        length
    }
}

impl Encodable for Transactions {
    fn length(&self) -> usize {
        let mut length = self.transactions_payload_length();
        length += length_of_length(length);
        length
    }
    fn encode(&self, out: &mut dyn bytes::BufMut) {
        let header = Header {
            list: true,
            payload_length: self.transactions_payload_length(),
        };
        header.encode(out);

        for tx_response in &self.0 {
            match tx_response {
                TypedTransaction::Legacy(tx) => tx.encode(out),
                TypedTransaction::EIP2930(tx) => {
                    let tx_header = Header {
                        list: false,
                        payload_length: tx.length() + 1,
                    };

                    tx_header.encode(out);
                    out.put_u8(0x01);
                    tx.encode(out);
                }
                TypedTransaction::EIP1559(tx) => {
                    let tx_header = Header {
                        list: false,
                        payload_length: tx.length() + 1,
                    };

                    tx_header.encode(out);
                    out.put_u8(0x02);
                    tx.encode(out);
                }
            }
        }
    }
}

impl Decodable for Transactions {
    fn decode(buf: &mut &[u8]) -> Result<Self, fastrlp::DecodeError> {
        let mut txs = Vec::new();
        // PooledTransactions always starts with a list header
        let _header = Header::decode(buf)?;
        while !buf.is_empty() {
            // decode the first byte of the header if it exists - if the element is a string (first
            // byte < 0xc0), then the element is a non-legacy transaction.
            // The header is only removed if the transaction is not legacy, since legacy
            // transaction decoding will automatically remove the header.
            if buf[0] < 0xc0 {
                let _header = Header::decode(buf)?;
            }
            txs.push(TypedTransaction::decode(buf)?);
        }
        Ok(Transactions(txs))
    }
}

impl From<Vec<TypedTransaction>> for Transactions {
    fn from(txs: Vec<TypedTransaction>) -> Self {
        Transactions(txs)
    }
}

impl From<Transactions> for Vec<TypedTransaction> {
    fn from(txs: Transactions) -> Self {
        txs.0
    }
}

/// This informs peers of transaction hashes for transactions that have appeared on the network,
/// but have not been included in a block.
#[derive(Clone, Debug, PartialEq, Eq, RlpEncodableWrapper, RlpDecodableWrapper)]
pub struct NewPooledTransactionHashes(pub Vec<[u8; 32]>);

// #[cfg(test)]
// mod test {
//     // TODO: finish test
//     #[test]
//     fn decode_new_block_network() {
//         // let data = hex!("f90267f90260f9025ba0b1a42b7dbdabaea62f7ce570df72e99f109c926630631cb2d4a6825feefed8a7a01dcc4de8dec75d7aab85b567b6ccd41ad312451b948a7413f0a142fd40d49347940000000000000000000000000000000000000000a0d8ba3228ed6f115607c92c8cdd73c7f32fe2b3438bb87df4f34b87849b5151daa056e81f171bcc55a6ff8345e692c0f86e5b48e01b996cadc001622fb5e363b421a056e81f171bcc55a6ff8345e692c0f86e5b48e01b996cadc001622fb5e363b421b901000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000001830129fe8401c9c380808462bb2863b861d683010a12846765746886676f312e3138856c696e7578000000000000000000baa22ae014fc964e03af76696590f9b94a6c1bdb76235d446c16348d946c88e31fd922d15ee4eb7f37853ed5dcc9761012ef6306bd5eb7625a60e2b684afc10401a0000000000000000000000000000000000000000000000000000000000000000088000000000000000007c0c08301c5c6");
//     }
// }
