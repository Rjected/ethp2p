//! Construct a [`Status`](ethp2p::Status) and print out its encoding, then decode the [`Status`]
//! message from bytes.

use eyre::Result;
use hex_literal::hex;
use ruint::uint;
use ethp2p::{Status, EthVersion};
use foundry_config::Chain;
use fastrlp::{Encodable, Decodable};
use anvil::Hardfork;

const ETH_GENESIS = hex!("d4e56740f876aef8c010b86a40d5f56745a118d0906a34e69aec8c0db1cb8fa3");

fn main() -> Result<()> {
    let status = Status {
        version: EthVersion::Eth67 as u8,
        chain: Chain::Id(1),
        total_difficulty: uint!(54928867412924629891081_U256),
        blockhash: hex!("6890edf8ad6900a5472c2a7ee3ef795f020ef6f907afb7f4ebf6a92d6aeb1804"),
        genesis: ETH_GENESIS,
        forkid: Hardfork::Latest.fork_id(),
    };

    println!("Encoding the following status message: {:?}", status);

    let mut encoded_status = vec![];
    status.encode(encoded_status);

    println!("The RLP encoded status message: {}", hex::encode(encoded_status));

    let decoded_status = Status::decode(&mut encoded_status)?;
    assert_eq!(decoded_status, status);
}
