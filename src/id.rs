use near_sdk::borsh::{BorshDeserialize, BorshSerialize};
use near_sdk::serde::{ser::SerializeStruct, Deserialize, Serialize, Serializer};
use std::convert::TryFrom;

/// A checksum as bytes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, BorshSerialize, BorshDeserialize)]
pub struct Checksum(pub Vec<u8>);

impl ToString for Checksum {
    fn to_string(&self) -> String {
        hex::encode(&self.0)
    }
}

/// A version for the data included.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize, BorshSerialize, BorshDeserialize)]
pub struct Version {
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
        let Some(value) = value.strip_prefix('v') else { return Err(error::VersionError::UnusualVersion) };
        let parts: Vec<&str> = value.split_terminator('.').collect();
        // Check to ensure we have 3 parts.
        if parts.len() != 3 {
            return Err(error::VersionError::UnusualVersion);
        }

        let major = parts[0]
            .parse::<u32>()
            .map_err(error::VersionError::ParseInt);
        let minor = parts[1]
            .parse::<u32>()
            .map_err(error::VersionError::ParseInt);
        let patch = parts[2]
            .parse::<u32>()
            .map_err(error::VersionError::ParseInt);

        Ok(Self {
            major: major?,
            minor: minor?,
            patch: patch?,
        })
    }
}

impl TryFrom<&str> for Version {
    type Error = error::VersionError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        // Split string value into parts, seperated by `.`
        let Some(value) = value.strip_prefix('v') else { return Err(error::VersionError::UnusualVersion) };
        let parts: Vec<&str> = value.split_terminator('.').collect();
        // Check to ensure we have 3 parts.
        if parts.len() != 3 {
            return Err(error::VersionError::UnusualVersion);
        }

        let major = parts[0]
            .parse::<u32>()
            .map_err(error::VersionError::ParseInt);
        let minor = parts[1]
            .parse::<u32>()
            .map_err(error::VersionError::ParseInt);
        let patch = parts[2]
            .parse::<u32>()
            .map_err(error::VersionError::ParseInt);

        Ok(Self {
            major: major?,
            minor: minor?,
            patch: patch?,
        })
    }
}

impl ToString for Version {
    fn to_string(&self) -> String {
        format!("v{}.{}.{}", self.major, self.minor, self.patch)
    }
}

/// The Id of checksum data.
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct Id {
    /// The version of the data.
    version: Version,
    /// The blake2b checksum of the data.
    checksum: Checksum,
}

impl Id {
    #[must_use]
    pub const fn new(version: Version, checksum: Checksum) -> Self {
        Self { version, checksum }
    }
}

impl Serialize for Id {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("Id", 2)?;
        s.serialize_field("version", &self.version.to_string())?;
        s.serialize_field("checksum", &self.checksum.to_string())?;
        s.end()
    }
}

impl TryFrom<String> for Id {
    type Error = error::IdError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::try_from(&value[..])
    }
}

impl TryFrom<&str> for Id {
    type Error = error::IdError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
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
        let version = Version::try_from(parts[0])?;
        let checksum = Checksum(hex::decode(parts[1])?);

        Ok(Self { version, checksum })
    }
}

impl ToString for Id {
    fn to_string(&self) -> String {
        let version = self.version.to_string();
        let checksum = self.checksum.to_string();
        format!("{version}-{checksum}")
    }
}

#[derive(Serialize, BorshSerialize, Eq, PartialEq, BorshDeserialize)]
pub struct IdStatus {
    pub id: Id,
    pub status: Status,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, BorshSerialize, BorshDeserialize)]
pub enum Status {
    Released,
    Yanked,
}

pub mod error {
    use std::num::ParseIntError;
    use thiserror::Error;

    #[derive(Error, Debug)]
    pub enum VersionError {
        #[error("error must fit into a u32")]
        ParseInt(#[from] ParseIntError),
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
        let version_str = String::new();
        matches!(
            Version::try_from(version_str),
            Err(error::VersionError::UnusualVersion)
        );
    }

    #[test]
    fn test_bad_version() {
        let version_str = "v1.0".to_string();
        matches!(
            Version::try_from(version_str),
            Err(error::VersionError::UnusualVersion)
        );
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
