use std::{error::Error, fmt::Display};

use super::item::GitItem;

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
pub struct GitError {
    pub message: String,
}

impl Display for GitError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.message)
    }
}

impl Error for GitError {}

unsafe impl Send for GitError {}

unsafe impl Sync for GitError {}

pub trait Git {
    /// Gets all the items in the current git repository
    fn get_items(&self) -> Result<Vec<GitItem>, GitError>;

    /// Adds the given item to the git repository
    fn add(&self, item: &mut GitItem) -> Result<GitError, String>;

    /// Removes the given item from the git repository
    fn remove(&self, item: &mut GitItem) -> Result<GitError, String>;
}
