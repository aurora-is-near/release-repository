use std::convert::TryFrom;
use std::fmt::{Display, Formatter, Pointer};
use std::str::FromStr;
use near_sdk::borsh::{BorshSerialize, BorshDeserialize};

pub mod error {
    use thiserror::Error;

    #[derive(Error, Debug)]
    pub enum VersionError {
        #[error("error must fit into a u32")]
        StdError(#[from] std::io::Error),
        #[error("unusual version provided, expected `vX.Y.Z`")]
        UnusualVersion,
    }

    #[derive(Error, Debug)]
    pub enum IdError {
        #[error(transparent)]
        FromHexError(#[from] hex::FromHexError),
        #[error(transparent)]
        Version(#[from] VersionError),
        #[error("missing `v` as a prefix for version`")]
        MissingVPrefix,
        #[error("id is incorrect, expected `vX.Y.Z-<hash>`")]
        UnusualId,
        #[error("hash length must be 32 bytes")]
        HashLen,
    }
}

/// A checksum as bytes.
pub type Checksum = Vec<u8>;

/// A version for the data included.
#[derive(Debug, Copy, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub(crate) struct Version {
    /// The major version.
    major: u32,
    /// The minor version.
    minor: u32,
    /// The patch version.
    patch: u32,
}

impl TryFrom<String> for Version {
    type Error = error::VersionError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        // Split string value into parts, seperated by `.`
        let value = match value.strip_prefix('v') {
            Some(v) => v,
            None => return Err(error::VersionError::UnusualVersion),
        };

        let parts: Vec<u32> = value
            .split_terminator('.')
            // TODO: remove unwrap
            .map(|part| {
                u32::from_str(part).unwrap()
            })
            .collect();

        // Check to ensure we have 3 parts.
        if parts.len() != 3 {
            return Err(error::VersionError::UnusualVersion);
        }

        Ok(Version {
            major: parts[0],
            minor: parts[1],
            patch: parts[2],
        })
    }
}

impl ToString for Version {
    fn to_string(&self) -> String {
        format!("{}.{}.{}", self.major, self.minor, self.patch)
    }
}

/// The Id of checksum data.
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub(crate) struct Id {
    /// The version of the data.
    version: Version,
    /// The blake2b checksum of the data.
    checksum: Checksum,
}

impl Id {
    pub(crate) fn new(version: Version, checksum: Checksum) -> Self {
        Id { version, checksum }
    }
}

impl TryFrom<String> for Id {
    type Error = error::IdError;

    fn try_from(value: String) -> Result<Id, Self::Error> {
        let parts: Vec<&str> = value.split_terminator('-').collect();

        // Check to ensure only two parts exist.
        if parts.len() != 2 {
            return Err(error::IdError::UnusualId);
        }

        // Check to ensure that the first part starts with a 'v' for
        // version.
        if !parts[0].starts_with('v') {
            return Err(error::IdError::MissingVPrefix);
        }

        // Check to ensure that the 2nd part is exactly 64 bytes long.
        if parts[1].len() != 64 {
            return Err(error::IdError::HashLen);
        }

        // TODO: Not happy with using `to_string` here.
        let version = Version::try_from(parts[0].to_string())?;
        let checksum = hex::decode(parts[1])?;

        Ok(Id { version, checksum })
    }
}

impl ToString for Id {
    fn to_string(&self) -> String {
        let version = self.version.to_string();
        let checksum = hex::encode(&self.checksum);
        format!("{version}-{checksum}")
    }
}

#[derive(BorshSerialize, BorshDeserialize)]
pub(crate) struct IdStatus {
    pub id: Id,
    pub yanked: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version() {
        let version_str = "v1.2.3".to_string();
        let version = Version::try_from(version_str).unwrap();
        let expected = Version {
            major: 1,
            minor: 2,
            patch: 3,
        };

        assert_eq!(version, expected);
    }

    #[test]
    fn test_blank_version() {
        let version_str = "".to_string();
        matches!(Version::try_from(version_str), Err(error::VersionError::UnusualVersion));
    }

    #[test]
    fn test_bad_version() {
        let version_str = "v1.0".to_string();
        matches!(Version::try_from(version_str), Err(error::VersionError::UnusualVersion));
    }
    
    #[test]
    fn test_id() {
        let version = "v1.2.3".to_string();
        let checksum = "b94d27b9934d3e08a52e52d7da7dabfac484efe37a5380ee9088f7ace2efcde9";
        let id_string = format!("{version}-{checksum}");
        Id::try_from(id_string).unwrap();
    }

    #[test]
    fn test_bad_checksum_id() {
        let version = "v1.2.3".to_string();
        let checksum = "b94d27b9934d3e08a52e52d7da7dabfac484efe37a5380ee9088f7ace2";
        let id_string = format!("{version}-{checksum}");
        matches!(Id::try_from(id_string), Err(error::IdError::HashLen));
    }
}
