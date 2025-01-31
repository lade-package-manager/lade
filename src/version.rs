use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::cmp::Ordering;
use std::str::FromStr;

#[derive(Debug, Clone)]
pub struct Version {
    major: u32,
    minor: u32,
    patch: u32,
}

#[derive(Debug)]
pub struct ParseVersionError;

impl FromStr for Version {
    type Err = ParseVersionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split('.').collect();
        match parts.len() {
            3 => {
                let major = parts[0].parse::<u32>().map_err(|_| ParseVersionError)?;
                let minor = parts[1].parse::<u32>().map_err(|_| ParseVersionError)?;
                let patch = parts[2].parse::<u32>().map_err(|_| ParseVersionError)?;
                Ok(Self {
                    major,
                    minor,
                    patch,
                })
            }
            2 => {
                let major = parts[0].parse::<u32>().map_err(|_| ParseVersionError)?;
                let minor = parts[1].parse::<u32>().map_err(|_| ParseVersionError)?;
                Ok(Self {
                    major,
                    minor,
                    patch: 0,
                }) // patchはデフォルトで0
            }
            1 => {
                let major = parts[0].parse::<u32>().map_err(|_| ParseVersionError)?;
                Ok(Self {
                    major,
                    minor: 0,
                    patch: 0,
                }) // minorとpatchはデフォルトで0
            }
            _ => Err(ParseVersionError), // 無効な形式
        }
    }
}

impl std::fmt::Display for Version {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}.{}", self.major, self.minor, self.patch)
    }
}

impl PartialOrd for Version {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Version {
    fn cmp(&self, other: &Self) -> Ordering {
        self.major
            .cmp(&other.major)
            .then(self.minor.cmp(&other.minor))
            .then(self.patch.cmp(&other.patch))
    }
}

impl PartialEq for Version {
    fn eq(&self, other: &Self) -> bool {
        self.major == other.major && self.minor == other.minor && self.patch == other.patch
    }
}

impl Eq for Version {}

impl Serialize for Version {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for Version {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Version::from_str(&s).map_err(|_| serde::de::Error::custom("Cannot parse version pattern"))
    }
}
