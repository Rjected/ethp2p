use serde::{Deserialize, Serialize};
/// The `eth` protocol version.
#[repr(u8)]
#[derive(Clone, Copy, Debug, Hash, Serialize, Deserialize, PartialEq, Eq)]
pub enum EthVersion {
    Eth66 = 66,
    Eth67 = 67,
}

impl TryFrom<String> for EthVersion {
    type Error = String;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        match s.as_str() {
            "66" => Ok(EthVersion::Eth66),
            "67" => Ok(EthVersion::Eth67),
            _ => Err(format!("Unknown version: {}", s)),
        }
    }
}

impl From<EthVersion> for u8 {
    #[inline]
    fn from(v: EthVersion) -> u8 {
        v as u8
    }
}
