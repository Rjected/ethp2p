use super::forkid::ForkId;
use ethers::types::U256;
use foundry_config::Chain;
use open_fastrlp::{RlpDecodable, RlpEncodable};
use std::fmt::{Debug, Display};

/// The status message is used in the eth protocol handshake to ensure that peers are on the same
/// network and are following the same fork.
/// The total difficulty and best block hash are used to identify whether or not the requesting
/// client should be sent historical blocks for a full blockchain sync.
///
/// When performing a handshake, the total difficulty is not guaranteed to correspond to the block
/// hash. This information should be treated as untrusted.
#[derive(Copy, Clone, PartialEq, Eq, RlpEncodable, RlpDecodable)]
pub struct Status {
    /// The current protocol version. For example, peers running `eth/66` would have a version of
    /// 66.
    pub version: u8,

    /// The chain id, as introduced in
    /// [EIP155](https://eips.ethereum.org/EIPS/eip-155#list-of-chain-ids).
    pub chain: Chain,

    /// Total difficulty of the best chain.
    pub total_difficulty: U256,

    /// The highest difficulty block hash the peer has seen
    pub blockhash: [u8; 32],

    /// The genesis hash of the peer's chain.
    pub genesis: [u8; 32],

    /// The fork identifier, a [CRC32
    /// checksum](https://en.wikipedia.org/wiki/Cyclic_redundancy_check#CRC-32_algorithm) for
    /// identifying the peer's fork as defined by
    /// [EIP-2124](https://github.com/ethereum/EIPs/blob/master/EIPS/eip-2124.md).
    /// This was added in [`eth/64`](https://eips.ethereum.org/EIPS/eip-2364)
    pub forkid: ForkId,
}

// TODO: Determine if it's worth wrapping or aliasing [u8; 32] across these types, to help derive
// traits like this rather than having to manually implement them.
impl Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let hexed_blockhash = hex::encode(self.blockhash);
        let hexed_genesis = hex::encode(self.genesis);
        write!(
            f,
            "Status {{ version: {}, chain: {}, total_difficulty: {}, blockhash: {}, genesis: {}, forkid: {:X?} }}",
            self.version,
            self.chain,
            self.total_difficulty,
            hexed_blockhash,
            hexed_genesis,
            self.forkid
        )
    }
}

impl Debug for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let hexed_blockhash = hex::encode(self.blockhash);
        let hexed_genesis = hex::encode(self.genesis);
        if f.alternate() {
            write!(
                f,
                "Status {{\n\tversion: {:?},\n\tchain: {:?},\n\ttotal_difficulty: {:?},\n\tblockhash: {},\n\tgenesis: {},\n\tforkid: {:X?}\n}}",
                self.version,
                self.chain,
                self.total_difficulty,
                hexed_blockhash,
                hexed_genesis,
                self.forkid
            )
        } else {
            write!(
                f,
                "Status {{ version: {:?}, chain: {:?}, total_difficulty: {:?}, blockhash: {}, genesis: {}, forkid: {:X?} }}",
                self.version,
                self.chain,
                self.total_difficulty,
                hexed_blockhash,
                hexed_genesis,
                self.forkid
            )
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::forkid::{ForkHash, ForkId};
    use ethers::prelude::Chain as NamedChain;
    use foundry_config::Chain;
    use hex_literal::hex;
    use open_fastrlp::{Decodable, Encodable};

    use crate::{EthVersion, Status};

    #[test]
    fn encode_eth_status_message() {
        let expected = hex!("f85643018a07aac59dabcdd74bc567a0feb27336ca7923f8fab3bd617fcb6e75841538f71c1bcfc267d7838489d9e13da0d4e56740f876aef8c010b86a40d5f56745a118d0906a34e69aec8c0db1cb8fa3c684b715077d80");
        let status = Status {
            version: EthVersion::Eth67 as u8,
            // ethers versions arent the same due to patches, so using Id here
            chain: Chain::Named(NamedChain::Mainnet),
            // total_difficulty: Uint::from(36206751599115524359527u128),
            total_difficulty: ethers::types::U256::from(36206751599115524359527u128),
            blockhash: hex!("feb27336ca7923f8fab3bd617fcb6e75841538f71c1bcfc267d7838489d9e13d"),
            genesis: hex!("d4e56740f876aef8c010b86a40d5f56745a118d0906a34e69aec8c0db1cb8fa3"),
            forkid: ForkId {
                hash: ForkHash([0xb7, 0x15, 0x07, 0x7d]),
                next: 0,
            },
        };

        let mut rlp_status = vec![];
        status.encode(&mut rlp_status);
        assert_eq!(rlp_status, expected);
    }

    #[test]
    fn decode_eth_status_message() {
        let data = hex!("f85643018a07aac59dabcdd74bc567a0feb27336ca7923f8fab3bd617fcb6e75841538f71c1bcfc267d7838489d9e13da0d4e56740f876aef8c010b86a40d5f56745a118d0906a34e69aec8c0db1cb8fa3c684b715077d80");
        let expected = Status {
            version: EthVersion::Eth67 as u8,
            // ethers versions arent the same due to patches, so using Id here
            chain: Chain::Named(NamedChain::Mainnet),
            total_difficulty: ethers::types::U256::from(36206751599115524359527u128),
            blockhash: hex!("feb27336ca7923f8fab3bd617fcb6e75841538f71c1bcfc267d7838489d9e13d"),
            genesis: hex!("d4e56740f876aef8c010b86a40d5f56745a118d0906a34e69aec8c0db1cb8fa3"),
            forkid: ForkId {
                hash: ForkHash([0xb7, 0x15, 0x07, 0x7d]),
                next: 0,
            },
        };
        let status = Status::decode(&mut &data[..]).unwrap();
        assert_eq!(status, expected);
    }

    #[test]
    fn encode_network_status_message() {
        let expected = hex!("f850423884024190faa0f8514c4680ef27700751b08f37645309ce65a449616a3ea966bf39dd935bb27ba00d21840abff46b96c84b2ac9e10e4f5cdaeb5693cb665db62a2f3b02d2d57b5bc6845d43d2fd80");
        let status = Status {
            version: EthVersion::Eth66 as u8,
            chain: Chain::Named(NamedChain::BinanceSmartChain),
            total_difficulty: ethers::types::U256::from(37851386u64),
            blockhash: hex!("f8514c4680ef27700751b08f37645309ce65a449616a3ea966bf39dd935bb27b"),
            genesis: hex!("0d21840abff46b96c84b2ac9e10e4f5cdaeb5693cb665db62a2f3b02d2d57b5b"),
            forkid: ForkId {
                hash: ForkHash([0x5d, 0x43, 0xd2, 0xfd]),
                next: 0,
            },
        };

        let mut rlp_status = vec![];
        status.encode(&mut rlp_status);
        assert_eq!(rlp_status, expected);
    }

    #[test]
    fn decode_network_status_message() {
        let data = hex!("f850423884024190faa0f8514c4680ef27700751b08f37645309ce65a449616a3ea966bf39dd935bb27ba00d21840abff46b96c84b2ac9e10e4f5cdaeb5693cb665db62a2f3b02d2d57b5bc6845d43d2fd80");
        let expected = Status {
            version: EthVersion::Eth66 as u8,
            chain: Chain::Named(NamedChain::BinanceSmartChain),
            total_difficulty: ethers::types::U256::from(37851386u64),
            blockhash: hex!("f8514c4680ef27700751b08f37645309ce65a449616a3ea966bf39dd935bb27b"),
            genesis: hex!("0d21840abff46b96c84b2ac9e10e4f5cdaeb5693cb665db62a2f3b02d2d57b5b"),
            forkid: ForkId {
                hash: ForkHash([0x5d, 0x43, 0xd2, 0xfd]),
                next: 0,
            },
        };
        let status = Status::decode(&mut &data[..]).unwrap();
        assert_eq!(status, expected);
    }

    #[test]
    fn decode_another_network_status_message() {
        let data = hex!("f86142820834936d68fcffffffffffffffffffffffffdeab81b8a0523e8163a6d620a4cc152c547a05f28a03fec91a2a615194cb86df9731372c0ca06499dccdc7c7def3ebb1ce4c6ee27ec6bd02aee570625ca391919faf77ef27bdc6841a67ccd880");
        let expected = Status {
            version: EthVersion::Eth66 as u8,
            chain: Chain::Id(2100),
            total_difficulty: ethers::types::U256::from(
                "0x000000000000000000000000006d68fcffffffffffffffffffffffffdeab81b8",
            ),
            blockhash: hex!("523e8163a6d620a4cc152c547a05f28a03fec91a2a615194cb86df9731372c0c"),
            genesis: hex!("6499dccdc7c7def3ebb1ce4c6ee27ec6bd02aee570625ca391919faf77ef27bd"),
            forkid: ForkId {
                hash: ForkHash([0x1a, 0x67, 0xcc, 0xd8]),
                next: 0,
            },
        };
        let status = Status::decode(&mut &data[..]).unwrap();
        assert_eq!(status, expected);
    }
}
