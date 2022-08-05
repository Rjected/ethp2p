use thiserror::Error;
use std::str::FromStr;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Error)]
#[error("Unknown eth protocol version: {0}")]
pub struct ParseVersionError(String);

/// The `eth` protocol version.
#[repr(u8)]
#[derive(Clone, Copy, Debug, Hash, Serialize, Deserialize, PartialEq, Eq)]
pub enum EthVersion {
    Eth66 = 66,
    Eth67 = 67,
}

/// Allow for converting from a `&str` to an `EthVersion`.
///
/// # Example
/// ```
/// use ethp2p::EthVersion;
/// use std::convert::TryFrom;
///
/// let version = EthVersion::try_from("67").unwrap();
/// assert_eq!(version, EthVersion::Eth67);
/// ```
impl TryFrom<&str> for EthVersion {
    type Error = ParseVersionError;

    #[inline]
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            "66" => Ok(EthVersion::Eth66),
            "67" => Ok(EthVersion::Eth67),
            _ => Err(ParseVersionError(s.to_string())),
        }
    }
}

/// Allow for converting from a u8 to an `EthVersion`.
///
/// # Example
/// ```
/// use ethp2p::EthVersion;
/// use std::convert::TryFrom;
///
/// let version = EthVersion::try_from(67).unwrap();
/// assert_eq!(version, EthVersion::Eth67);
/// ```
impl TryFrom<u8> for EthVersion {
    type Error = ParseVersionError;

    #[inline]
    fn try_from(u: u8) -> Result<Self, Self::Error> {
        match u {
            66 => Ok(EthVersion::Eth66),
            67 => Ok(EthVersion::Eth67),
            _ => Err(ParseVersionError(u.to_string())),
        }
    }
}

impl FromStr for EthVersion {
    type Err = ParseVersionError;

    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        EthVersion::try_from(s)
    }
}

impl From<EthVersion> for u8 {
    #[inline]
    fn from(v: EthVersion) -> u8 {
        v as u8
    }
}

impl From<EthVersion> for &'static str {
    #[inline]
    fn from(v: EthVersion) -> &'static str {
        match v {
            EthVersion::Eth66 => "66",
            EthVersion::Eth67 => "67",
        }
    }
}

#[cfg(test)]
mod test {
    use super::{EthVersion, ParseVersionError};
    use std::convert::TryFrom;
    use std::string::ToString;

    #[test]
    fn test_eth_version_try_from_str() {
        assert_eq!(EthVersion::Eth66, EthVersion::try_from("66").unwrap());
        assert_eq!(EthVersion::Eth67, EthVersion::try_from("67").unwrap());
        assert_eq!(
            Err(ParseVersionError("68".to_string())),
            EthVersion::try_from("68")
        );
    }

    #[test]
    fn test_eth_version_from_str() {
        assert_eq!(EthVersion::Eth66, "66".parse().unwrap());
        assert_eq!(EthVersion::Eth67, "67".parse().unwrap());
        assert_eq!(
            Err(ParseVersionError("68".to_string())),
            "68".parse::<EthVersion>()
        );
    }
}
