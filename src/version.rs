use serde::{Deserialize, Serialize};
/// The `eth` protocol version.
#[repr(u8)]
#[derive(Clone, Copy, Debug, Hash, Serialize, Deserialize, PartialEq, Eq)]
pub enum EthVersion {
    Eth66 = 66,
    Eth67 = 67,
}
