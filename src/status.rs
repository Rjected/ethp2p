use ethereum_forkid::ForkId;
use fastrlp::{RlpDecodable, RlpEncodable};
use foundry_config::Chain;
use ruint::Uint;

/// The status message is used in the eth protocol handshake to ensure that peers are on the same
/// network and are following the same fork.
/// The total difficulty and best block hash are used to identify whether or not the requesting
/// client should be sent historical blocks for a full blockchain sync.
///
/// When performing a handshake, the total difficulty is not guaranteed to correspond to the block
/// hash. This information should be treated as untrusted.
#[derive(Clone, Debug, PartialEq, Eq, RlpEncodable, RlpDecodable)]
pub struct Status {
    /// The current protocol version. For example, peers running eth/65 would have a version of 65.
    pub version: u8,

    /// The chain id, as introduced in
    /// [EIP155](https://eips.ethereum.org/EIPS/eip-155#list-of-chain-ids).
    pub chain: Chain,

    /// Total difficulty of the best chain.
    /// The ethereum difficulty is unlikely to exceed 128 bits, but is currently over 64 bits, so
    /// this is represented as a 128 bit integer.
    pub total_difficulty: Uint<128, 2>,

    /// The highest difficulty block hash the peer has seen
    pub blockhash: [u8; 32],

    /// The genesis hash of the peer's chain
    pub genesis: [u8; 32],

    /// The fork identifier, a [CRC32
    /// checksum](https://en.wikipedia.org/wiki/Cyclic_redundancy_check#CRC-32_algorithm) for
    /// identifying the peer's fork as defined by
    /// [EIP-2124](https://github.com/ethereum/EIPs/blob/master/EIPS/eip-2124.md).
    pub forkid: ForkId,
}

#[cfg(test)]
mod tests {
    use ethereum_forkid::{ForkHash, ForkId};
    use fastrlp::Encodable;
    use foundry_config::Chain;
    use hex_literal::hex;
    use ruint::Uint;

    use crate::{EthVersion, Status};

    #[test]
    fn create_status_message() {
        let status = Status {
            version: EthVersion::Eth67 as u8,
            // ethers versions arent the same due to patches, so using Id here
            chain: Chain::Id(1),
            total_difficulty: Uint::from(36206751599115524359527u128),
            blockhash: hex!("feb27336ca7923f8fab3bd617fcb6e75841538f71c1bcfc267d7838489d9e13d"),
            genesis: hex!("d4e56740f876aef8c010b86a40d5f56745a118d0906a34e69aec8c0db1cb8fa3"),
            forkid: ForkId {
                hash: ForkHash([0xb7, 0x15, 0x07, 0x7d]),
                next: 0,
            },
        };

        let mut rlp_status = vec![];
        status.encode(&mut rlp_status);
        println!("here is the rlp status: {:X?}", rlp_status);
    }
}
