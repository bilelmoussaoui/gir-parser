use std::str::FromStr;

use serde::Deserialize;
use xmlserde::XmlValue;

#[derive(Debug, Default, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Version(u16, Option<u16>, Option<u16>);

impl Version {
    pub fn new(major: u16, minor: u16, patch: u16) -> Self {
        Self(major, Some(minor), Some(patch))
    }

    pub fn major(self) -> u16 {
        self.0
    }

    pub fn minor(self) -> Option<u16> {
        self.1
    }

    pub fn patch(self) -> Option<u16> {
        self.2
    }
}

impl<'de> Deserialize<'de> for Version {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let str = <String as Deserialize>::deserialize(deserializer)?;
        Self::from_str(&str).map_err(serde::de::Error::custom)
    }
}

impl FromStr for Version {
    type Err = String;

    /// Parse a `Version` from a string.
    /// Currently always return Ok
    fn from_str(s: &str) -> Result<Self, String> {
        if s.contains('.') {
            let mut parts = s
                .splitn(4, '.')
                .map(str::parse)
                .take_while(Result::is_ok)
                .map(Result::unwrap);
            Ok(Self(parts.next().unwrap_or(0), parts.next(), parts.next()))
        } else {
            let val = s.parse::<u16>();
            Ok(Self(val.unwrap_or(0), None, None))
        }
    }
}

impl XmlValue for Version {
    fn serialize(&self) -> String {
        self.to_string()
    }

    fn deserialize(s: &str) -> Result<Self, String> {
        Self::from_str(s)
    }
}

impl std::fmt::Display for Version {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        match *self {
            Self(major, None, None) => write!(f, "{major}"),
            Self(major, Some(minor), None) => write!(f, "{major}.{minor}"),
            Self(major, None, Some(patch)) => write!(f, "{major}.0.{patch}"),
            Self(major, Some(minor), Some(patch)) => write!(f, "{major}.{minor}.{patch}"),
        }
    }
}

impl PartialEq<str> for Version {
    fn eq(&self, other: &str) -> bool {
        Version::from_str(other) == Ok(*self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accessors() {
        let version = Version(5, Some(10), None);
        assert_eq!(version.major(), 5);
        assert_eq!(version.minor(), Some(10));
        assert_eq!(version.patch(), None);

        let version = Version(7, None, None);
        assert_eq!(version.major(), 7);
        assert_eq!(version.minor(), None);
        assert_eq!(version.patch(), None);

        let version = Version::new(1, 2, 3);
        assert_eq!(version.major(), 1);
        assert_eq!(version.minor(), Some(2));
        assert_eq!(version.patch(), Some(3));
    }

    #[test]
    fn from_str() {
        // Full version
        let version = Version::from_str("1.2.3").unwrap();
        assert_eq!(version, Version::new(1, 2, 3));

        // Major.minor
        let version = Version::from_str("2.5").unwrap();
        assert_eq!(version, Version(2, Some(5), None));

        // Major only with dot
        let version = Version::from_str("3.").unwrap();
        assert_eq!(version, Version(3, None, None));

        // Major only without dot
        let version = Version::from_str("4").unwrap();
        assert_eq!(version, Version(4, None, None));

        // Four parts (only first three are used)
        let version = Version::from_str("1.2.3.4").unwrap();
        assert_eq!(version, Version::new(1, 2, 3));

        // Invalid parts are ignored
        let version = Version::from_str("5.invalid.7").unwrap();
        assert_eq!(version, Version(5, None, None));

        // Completely invalid input defaults to 0
        let version = Version::from_str("invalid").unwrap();
        assert_eq!(version, Version(0, None, None));

        // Empty string defaults to 0
        let version = Version::from_str("").unwrap();
        assert_eq!(version, Version(0, None, None));

        assert_eq!(Version::new(1, 2, 3).to_string(), "1.2.3");
        assert_eq!(Version(2, Some(5), None).to_string(), "2.5");
        assert_eq!(Version(3, None, None).to_string(), "3");
        assert_eq!(Version(4, None, Some(6)).to_string(), "4.0.6");
    }

    #[test]
    fn eq_str() {
        // Test exact matches
        let version = Version::new(1, 2, 3);
        assert_eq!(version, *"1.2.3");
        assert!(version == *"1.2.3");

        // Test major.minor format
        let version = Version(2, Some(5), None);
        assert_eq!(version, *"2.5");
        assert!(version == *"2.5");

        // Test major only format
        let version = Version(3, None, None);
        assert_eq!(version, *"3");
        assert!(version == *"3");

        // Test non-matching versions
        let version = Version::new(1, 2, 3);
        assert_ne!(version, *"1.2.4");
        assert_ne!(version, *"2.2.3");
        assert_ne!(version, *"1.3.3");

        // Test invalid string comparison
        let version = Version::new(1, 2, 3);
        assert_ne!(version, *"invalid");
        assert_ne!(version, *"");
    }

    #[test]
    fn ordering() {
        // Major version comparison
        assert!(Version(2, None, None) > Version(1, None, None));
        assert!(Version(1, None, None) < Version(2, None, None));

        // Minor version comparison
        assert!(Version(1, Some(5), None) > Version(1, Some(3), None));
        assert!(Version(1, Some(2), None) < Version(1, Some(4), None));

        // Patch version comparison
        assert!(Version::new(1, 2, 5) > Version::new(1, 2, 3));
        assert!(Version::new(1, 2, 1) < Version::new(1, 2, 2));

        // None is less than Some
        assert!(Version(1, None, None) < Version(1, Some(0), None));
        assert!(Version(1, Some(2), None) < Version::new(1, 2, 0));

        // Equality
        assert_eq!(Version::new(1, 2, 3), Version::new(1, 2, 3));
        assert_eq!(Version(5, None, None), Version(5, None, None));
    }
}
