//! Encoding and decoding tests for [`NewPooledTransactions`]
use std::{
    fs,
    path::PathBuf,
};
use fastrlp::Decodable;
use ethp2p_rs::NewPooledTransactionHashes;

#[test]
fn decode_new_pooled_transaction_hashes_network() {
    // large test vector
    let network_data_path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("testdata/new_pooled_transactions_network_rlp");
    let data = fs::read_to_string(network_data_path).expect("Unable to read file");
    let hex_data = hex::decode(&data.trim()).unwrap();
    let _txs = NewPooledTransactionHashes::decode(&mut &hex_data[..]).unwrap();
}
