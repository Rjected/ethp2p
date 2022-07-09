use anvil_core::eth::{block::Block, transaction::TypedTransaction};
use fastrlp::{RlpDecodable, RlpDecodableWrapper, RlpEncodable, RlpEncodableWrapper};
use ruint::Uint;

/// This informs peers of new blocks that have appeared on the network.
#[derive(Clone, Debug, PartialEq, Eq, RlpEncodableWrapper, RlpDecodableWrapper)]
pub struct NewBlockHashes(
    /// New block hashes and the block number for each blockhash.
    /// Clients should request blocks using a [`GetBlockBodies`](crate::GetBlockBodies) message.
    pub Vec<BlockHashNumber>,
);

/// A block hash _and_ a block number.
#[derive(Clone, Debug, PartialEq, Eq, RlpEncodable, RlpDecodable)]
pub struct BlockHashNumber {
    /// The block hash
    pub hash: [u8; 32],
    /// The block number
    pub number: u64,
}

impl From<Vec<BlockHashNumber>> for NewBlockHashes {
    fn from(v: Vec<BlockHashNumber>) -> Self {
        NewBlockHashes(v)
    }
}

impl From<NewBlockHashes> for Vec<BlockHashNumber> {
    fn from(v: NewBlockHashes) -> Self {
        v.0
    }
}

/// A new block with the current total difficulty, which includes the difficulty of the returned
/// block.
#[derive(Clone, Debug, PartialEq, Eq, RlpEncodable, RlpDecodable)]
pub struct NewBlock {
    /// A new block.
    pub block: Block,
    /// The current total difficulty.
    pub td: Uint<128, 2>,
}

/// This informs peers of transactions that have appeared on the network and are not yet included
/// in a block.
#[derive(Clone, Debug, PartialEq, Eq, RlpEncodableWrapper, RlpDecodableWrapper)]
pub struct Transactions(
    /// New transactions for the peer to include in its mempool.
    pub Vec<TypedTransaction>,
);

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
pub struct NewPooledTransactionHashes(
    /// Transaction hashes for new transactions that have appeared on the network.
    /// Clients should request the transactions with the given hashes using a
    /// [`GetPooledTransactions`](crate::GetPooledTransactions) message.
    pub Vec<[u8; 32]>,
);

#[cfg(test)]
mod test {
    use crate::{Transactions, NewBlockHashes, BlockHashNumber};
    use anvil_core::eth::transaction::{LegacyTransaction, TransactionKind, TypedTransaction};
    use ethers::prelude::Signature;
    use fastrlp::{Decodable, Encodable};
    use hex_literal::hex;

    #[test]
    fn decode_transactions_network() {
        let data = hex!("f90382f9016e82015a85013f2ed0c0830224ae941b6c9c20693afde803b27f8782156c0f892abc2d80b9010438ed173900000000000000000000000000000000000000000000000000000003d4409a530000000000000000000000000000000000000000000000314a79d7d845deec1e00000000000000000000000000000000000000000000000000000000000000a00000000000000000000000002bb8351ad8b1acb7f81649a1688171afdddc3f6a0000000000000000000000000000000000000000000000000000000062c901f00000000000000000000000000000000000000000000000000000000000000002000000000000000000000000c9882def23bc42d53895b8361d0b1edc7570bc6a0000000000000000000000004a846d300f793752ee8bd579192c477130c4b3698193a005475649559ac86feae75bdc2aff8bd2c5a1bcaa4b48fc56427b1a10eb41a0aaa02ed0cce378d0a6efaa72d5e1183f6126d66a353c98421689f0e8c3d473d4d9bdf9020e82155285013f2ed0c0833a0e98943f32d3eaabdcb3a9fe8101e58cf40621b7d43e1580b901a44ef2ba160000000000000000000000000000000000000000000000000004fcbb43000000000000000000000000000000000000000000000000000000000000000000004000000000000000000000000000000000000000000000000000000000000000050000000000000000000027068840c6252e2e86e545defb6da98b2a0e26d8c1ba00000000000000000000271055d398326f99059ff775485246999027b31979550000000000000000000026f70d9de8e770dd12369099e0fad4903a7b42a907e90000000000000000000027104a846d300f793752ee8bd579192c477130c4b3690000000000000000000026f2088c46b98fc06c96469670059ac1bf453b4f886b000000000000000000002710c9882def23bc42d53895b8361d0b1edc7570bc6a0000000000000000000026f2796acba6556f70a3c5756a0d8fd0a10251c210500000000000000000000027100efb5fd2402a0967b92551d6af54de148504a1158000000000000000000000000000000000000000000000000000000000000001000000000000000000000000bb4cdb9cbd36b01bd1cbaebf2de08d9173bc095c8194a07b3ffd773850417791c32549f4f47319d41b05136ae8c6d855bf35cce9bf8e68a042c5cfa584281a342e1c0434ee2ef554dbaa4f16ec7efa0fb5cd1d52436d2475");
        let txs = Transactions::decode(&mut &data[..]).unwrap();
        let expected_txs: Transactions = vec![
            TypedTransaction::Legacy(
                LegacyTransaction {
                    nonce: 346.into(),
                    gas_price: 5355000000u64.into(),
                    gas_limit: 140462.into(),
                    kind: TransactionKind::Call(
                        hex!("1b6c9c20693afde803b27f8782156c0f892abc2d").into(),
                    ),
                    value: 0.into(),
                    input: hex!("38ed173900000000000000000000000000000000000000000000000000000003d4409a530000000000000000000000000000000000000000000000314a79d7d845deec1e00000000000000000000000000000000000000000000000000000000000000a00000000000000000000000002bb8351ad8b1acb7f81649a1688171afdddc3f6a0000000000000000000000000000000000000000000000000000000062c901f00000000000000000000000000000000000000000000000000000000000000002000000000000000000000000c9882def23bc42d53895b8361d0b1edc7570bc6a0000000000000000000000004a846d300f793752ee8bd579192c477130c4b369").into(),
                    signature: Signature {
                        r: hex!("05475649559ac86feae75bdc2aff8bd2c5a1bcaa4b48fc56427b1a10eb41a0aa").into(),
                        s: hex!("2ed0cce378d0a6efaa72d5e1183f6126d66a353c98421689f0e8c3d473d4d9bd").into(),
                        v: 147,
                    },
                }
            ),
            TypedTransaction::Legacy(
                LegacyTransaction {
                    nonce: 5458.into(),
                    gas_price: 5355000000u64.into(),
                    gas_limit: 3804824.into(),
                    kind: TransactionKind::Call(
                        hex!("3f32d3eaabdcb3a9fe8101e58cf40621b7d43e15").into()
                    ),
                    value: 0.into(),
                    input: hex!("4ef2ba160000000000000000000000000000000000000000000000000004fcbb43000000000000000000000000000000000000000000000000000000000000000000004000000000000000000000000000000000000000000000000000000000000000050000000000000000000027068840c6252e2e86e545defb6da98b2a0e26d8c1ba00000000000000000000271055d398326f99059ff775485246999027b31979550000000000000000000026f70d9de8e770dd12369099e0fad4903a7b42a907e90000000000000000000027104a846d300f793752ee8bd579192c477130c4b3690000000000000000000026f2088c46b98fc06c96469670059ac1bf453b4f886b000000000000000000002710c9882def23bc42d53895b8361d0b1edc7570bc6a0000000000000000000026f2796acba6556f70a3c5756a0d8fd0a10251c210500000000000000000000027100efb5fd2402a0967b92551d6af54de148504a1158000000000000000000000000000000000000000000000000000000000000001000000000000000000000000bb4cdb9cbd36b01bd1cbaebf2de08d9173bc095c").into(),
                    signature: Signature {
                        r: hex!("7b3ffd773850417791c32549f4f47319d41b05136ae8c6d855bf35cce9bf8e68").into(),
                        s: hex!("42c5cfa584281a342e1c0434ee2ef554dbaa4f16ec7efa0fb5cd1d52436d2475").into(),
                        v: 148,
                    },
                }
            ),
        ].into();
        assert_eq!(expected_txs, txs);
    }

    #[test]
    fn encode_transactions_network() {
        let expected = hex!("f90382f9016e82015a85013f2ed0c0830224ae941b6c9c20693afde803b27f8782156c0f892abc2d80b9010438ed173900000000000000000000000000000000000000000000000000000003d4409a530000000000000000000000000000000000000000000000314a79d7d845deec1e00000000000000000000000000000000000000000000000000000000000000a00000000000000000000000002bb8351ad8b1acb7f81649a1688171afdddc3f6a0000000000000000000000000000000000000000000000000000000062c901f00000000000000000000000000000000000000000000000000000000000000002000000000000000000000000c9882def23bc42d53895b8361d0b1edc7570bc6a0000000000000000000000004a846d300f793752ee8bd579192c477130c4b3698193a005475649559ac86feae75bdc2aff8bd2c5a1bcaa4b48fc56427b1a10eb41a0aaa02ed0cce378d0a6efaa72d5e1183f6126d66a353c98421689f0e8c3d473d4d9bdf9020e82155285013f2ed0c0833a0e98943f32d3eaabdcb3a9fe8101e58cf40621b7d43e1580b901a44ef2ba160000000000000000000000000000000000000000000000000004fcbb43000000000000000000000000000000000000000000000000000000000000000000004000000000000000000000000000000000000000000000000000000000000000050000000000000000000027068840c6252e2e86e545defb6da98b2a0e26d8c1ba00000000000000000000271055d398326f99059ff775485246999027b31979550000000000000000000026f70d9de8e770dd12369099e0fad4903a7b42a907e90000000000000000000027104a846d300f793752ee8bd579192c477130c4b3690000000000000000000026f2088c46b98fc06c96469670059ac1bf453b4f886b000000000000000000002710c9882def23bc42d53895b8361d0b1edc7570bc6a0000000000000000000026f2796acba6556f70a3c5756a0d8fd0a10251c210500000000000000000000027100efb5fd2402a0967b92551d6af54de148504a1158000000000000000000000000000000000000000000000000000000000000001000000000000000000000000bb4cdb9cbd36b01bd1cbaebf2de08d9173bc095c8194a07b3ffd773850417791c32549f4f47319d41b05136ae8c6d855bf35cce9bf8e68a042c5cfa584281a342e1c0434ee2ef554dbaa4f16ec7efa0fb5cd1d52436d2475");
        let txs: Transactions = vec![
            TypedTransaction::Legacy(
                LegacyTransaction {
                    nonce: 346.into(),
                    gas_price: 5355000000u64.into(),
                    gas_limit: 140462.into(),
                    kind: TransactionKind::Call(
                        hex!("1b6c9c20693afde803b27f8782156c0f892abc2d").into(),
                    ),
                    value: 0.into(),
                    input: hex!("38ed173900000000000000000000000000000000000000000000000000000003d4409a530000000000000000000000000000000000000000000000314a79d7d845deec1e00000000000000000000000000000000000000000000000000000000000000a00000000000000000000000002bb8351ad8b1acb7f81649a1688171afdddc3f6a0000000000000000000000000000000000000000000000000000000062c901f00000000000000000000000000000000000000000000000000000000000000002000000000000000000000000c9882def23bc42d53895b8361d0b1edc7570bc6a0000000000000000000000004a846d300f793752ee8bd579192c477130c4b369").into(),
                    signature: Signature {
                        r: hex!("05475649559ac86feae75bdc2aff8bd2c5a1bcaa4b48fc56427b1a10eb41a0aa").into(),
                        s: hex!("2ed0cce378d0a6efaa72d5e1183f6126d66a353c98421689f0e8c3d473d4d9bd").into(),
                        v: 147,
                    },
                }
            ),
            TypedTransaction::Legacy(
                LegacyTransaction {
                    nonce: 5458.into(),
                    gas_price: 5355000000u64.into(),
                    gas_limit: 3804824.into(),
                    kind: TransactionKind::Call(
                        hex!("3f32d3eaabdcb3a9fe8101e58cf40621b7d43e15").into()
                    ),
                    value: 0.into(),
                    input: hex!("4ef2ba160000000000000000000000000000000000000000000000000004fcbb43000000000000000000000000000000000000000000000000000000000000000000004000000000000000000000000000000000000000000000000000000000000000050000000000000000000027068840c6252e2e86e545defb6da98b2a0e26d8c1ba00000000000000000000271055d398326f99059ff775485246999027b31979550000000000000000000026f70d9de8e770dd12369099e0fad4903a7b42a907e90000000000000000000027104a846d300f793752ee8bd579192c477130c4b3690000000000000000000026f2088c46b98fc06c96469670059ac1bf453b4f886b000000000000000000002710c9882def23bc42d53895b8361d0b1edc7570bc6a0000000000000000000026f2796acba6556f70a3c5756a0d8fd0a10251c210500000000000000000000027100efb5fd2402a0967b92551d6af54de148504a1158000000000000000000000000000000000000000000000000000000000000001000000000000000000000000bb4cdb9cbd36b01bd1cbaebf2de08d9173bc095c").into(),
                    signature: Signature {
                        r: hex!("7b3ffd773850417791c32549f4f47319d41b05136ae8c6d855bf35cce9bf8e68").into(),
                        s: hex!("42c5cfa584281a342e1c0434ee2ef554dbaa4f16ec7efa0fb5cd1d52436d2475").into(),
                        v: 148,
                    },
                }
            ),
        ].into();
        let mut encoded = vec![];
        txs.encode(&mut encoded);
        let expected_str = hex::encode(expected);
        let encoded_str = hex::encode(encoded);
        assert_eq!(expected_str, encoded_str);
    }

    #[test]
    fn decode_new_block_hashes_network() {
        let data = hex!("e7e6a0fd3f0d4cb96a496ee7b77a238e48435600ce3337ce8f0309b7b57e91bfce89d6840127de96");
        let expected: NewBlockHashes = vec![
            BlockHashNumber {
                hash: hex!("fd3f0d4cb96a496ee7b77a238e48435600ce3337ce8f0309b7b57e91bfce89d6"),
                number: 19390102,
            },
        ].into();
        let decoded = NewBlockHashes::decode(&mut &data[..]).unwrap();
        assert_eq!(expected, decoded);
    }

    #[test]
    fn encode_new_block_hashes_network() {
        let expected = hex!("e7e6a0fd3f0d4cb96a496ee7b77a238e48435600ce3337ce8f0309b7b57e91bfce89d6840127de96");
        let hashes: NewBlockHashes = vec![
            BlockHashNumber {
                hash: hex!("fd3f0d4cb96a496ee7b77a238e48435600ce3337ce8f0309b7b57e91bfce89d6"),
                number: 19390102,
            },
        ].into();
        let mut encoded = vec![];
        hashes.encode(&mut encoded);
        let expected_str = hex::encode(expected);
        let encoded_str = hex::encode(encoded);
        assert_eq!(expected_str, encoded_str);
    }

}
