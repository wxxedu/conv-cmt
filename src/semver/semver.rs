use std::fmt::Display;

#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct SemVer {
    pub major: usize,
    pub minor: usize,
    pub patch: usize,
}

impl PartialOrd for SemVer {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if let Some(order) = self.major.partial_cmp(&other.major) {
            if order != std::cmp::Ordering::Equal {
                return Some(order);
            }
            if let Some(order) = self.minor.partial_cmp(&other.minor) {
                if order != std::cmp::Ordering::Equal {
                    return Some(order);
                }
                return self.patch.partial_cmp(&other.patch);
            }
        }
        None
    }
}

impl Ord for SemVer {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if let Some(order) = self.partial_cmp(other) {
            return order;
        }
        std::cmp::Ordering::Equal
    }
}

impl Display for SemVer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}.{}", self.major, self.minor, self.patch)
    }
}

impl SemVer {
    pub fn new(major: usize, minor: usize, patch: usize) -> Self {
        Self {
            major,
            minor,
            patch,
        }
    }

    pub fn from_str(version: &str) -> Option<Self> {
        let mut split = version.split('.');
        let major = split.next()?.parse::<usize>().ok()?;
        let minor = split.next()?.parse::<usize>().ok()?;
        let patch = split.next()?.parse::<usize>().ok()?;
        Some(Self {
            major,
            minor,
            patch,
        })
    }

    pub fn bump_major(&mut self) {
        self.major += 1;
        self.minor = 0;
        self.patch = 0;
    }

    pub fn bump_minor(&mut self) {
        self.minor += 1;
        self.patch = 0;
    }

    pub fn bump_patch(&mut self) {
        self.patch += 1;
    }
}
