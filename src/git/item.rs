use std::{fmt::Display, path::PathBuf};

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum ItemStatus {
    Unmodified,
    Modified,
    FileTypeChanged,
    Added,
    Deleted,
    Renamed,
    Copied,
    UpdatedButUnmerged,
}

unsafe impl Send for ItemStatus {}

unsafe impl Sync for ItemStatus {}

impl Display for ItemStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ItemStatus::Unmodified => write!(f, "Unmodified"),
            ItemStatus::Modified => write!(f, "Modified"),
            ItemStatus::FileTypeChanged => write!(f, "FileTypeChanged"),
            ItemStatus::Added => write!(f, "Added"),
            ItemStatus::Deleted => write!(f, "Deleted"),
            ItemStatus::Renamed => write!(f, "Renamed"),
            ItemStatus::Copied => write!(f, "Copied"),
            ItemStatus::UpdatedButUnmerged => write!(f, "UpdatedButUnmerged"),
        }
    }
}

impl ItemStatus {
    pub fn short_name(&self) -> &str {
        match self {
            ItemStatus::Unmodified => "U",
            ItemStatus::Modified => "M",
            ItemStatus::FileTypeChanged => "T",
            ItemStatus::Added => "A",
            ItemStatus::Deleted => "D",
            ItemStatus::Renamed => "R",
            ItemStatus::Copied => "C",
            ItemStatus::UpdatedButUnmerged => "X",
        }
    }
}

impl<T: AsRef<str>> From<T> for ItemStatus {
    fn from(status: T) -> Self {
        match status.as_ref() {
            "U" => ItemStatus::Unmodified,
            "M" => ItemStatus::Modified,
            "T" => ItemStatus::FileTypeChanged,
            "A" => ItemStatus::Added,
            "D" => ItemStatus::Deleted,
            "R" => ItemStatus::Renamed,
            "C" => ItemStatus::Copied,
            "X" => ItemStatus::UpdatedButUnmerged,
            _ => ItemStatus::Unmodified,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct GitItem {
    x: ItemStatus,
    y: ItemStatus,
    path: PathBuf,
}

impl Display for GitItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let x = self.x.short_name();
        let y = self.y.short_name();
        write!(f, "{}{} {}", x, y, self.path.display())
    }
}

impl GitItem {
    pub fn new(x: ItemStatus, y: ItemStatus, path: PathBuf) -> Self {
        Self { x, y, path }
    }

    pub fn is_staged(&self) -> bool {
        self.x != ItemStatus::Unmodified
    }
}
