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
        self.to_string() == other
    }
}
