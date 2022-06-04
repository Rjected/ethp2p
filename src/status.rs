use ethereum_forkid::ForkId;
use ethers::prelude::Chain;
use ruint::Uint;

/// The status message is used in the eth protocol handshake to ensure that peers are on the same
/// network and are following the same fork.
/// The total difficulty and best block hash are used to identify whether or not the requesting
/// client should be sent historical blocks for a full blockchain sync.
///
/// When performing a handshake, the total difficulty is not guaranteed to correspond to the block
/// hash. This information should be treated as untrusted.
pub struct Status {
    /// The current protocol version. For example, peers running eth/65 would have a version of 65.
    pub version: u8,

    /// The chain id, as introduced in [EIP155](https://eips.ethereum.org/EIPS/eip-155#list-of-chain-ids).
    pub chain: Chain,

    /// Total difficulty of the best chain.
    /// The ethereum difficulty is unlikely to exceed 128 bits, but is currently over 64 bits, so
    /// this is represented as a 128 bit integer.
    pub total_difficulty: Uint<128, 4>,

    /// The highest difficulty block hash the peer has seen
    pub blockhash: [u8; 32],

    /// The genesis hash of the peer's chain
    pub genesis: [u8; 32],

    /// The fork identifier, a [CRC32 checksum](https://en.wikipedia.org/wiki/Cyclic_redundancy_check#CRC-32_algorithm) for identifying the peer's fork as defined by [EIP-2124](https://github.com/ethereum/EIPs/blob/master/EIPS/eip-2124.md).
    pub forkid: ForkId,
}
