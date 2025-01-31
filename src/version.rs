use std::cmp::Ordering;

#[derive(Debug, Clone)]
pub struct Version {
    major: u32,
    minor: u32,
    patch: u32,
}

impl Version {
    // Versionを文字列からパース
    pub fn parse(version_str: &str) -> Option<Self> {
        let parts: Vec<&str> = version_str.split('.').collect();
        match parts.len() {
            3 => {
                let major = parts[0].parse::<u32>().ok()?;
                let minor = parts[1].parse::<u32>().ok()?;
                let patch = parts[2].parse::<u32>().ok()?;
                Some(Self {
                    major,
                    minor,
                    patch,
                })
            }
            2 => {
                let major = parts[0].parse::<u32>().ok()?;
                let minor = parts[1].parse::<u32>().ok()?;
                Some(Self {
                    major,
                    minor,
                    patch: 0,
                }) // patchはデフォルトで0
            }
            1 => {
                let major = parts[0].parse::<u32>().ok()?;
                Some(Self {
                    major,
                    minor: 0,
                    patch: 0,
                }) // minorとpatchはデフォルトで0
            }
            _ => None, // 無効な形式
        }
    }

    // バージョンを文字列に変換
    pub fn to_string(&self) -> String {
        format!("{}.{}.{}", self.major, self.minor, self.patch)
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
