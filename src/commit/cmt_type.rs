use std::fmt::Display;

use serde::{Deserialize, Serialize};

/// Represents a type of commit.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Default, Deserialize, Serialize,
)]
pub struct CommitType<'a> {
    /// The name of the commit type
    pub name: &'a str,
    /// The description of the commit type, to be shown in the terminal
    /// user interface
    pub description: Option<&'a str>,
}

impl<'a> Display for CommitType<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.name, self.description.unwrap_or(""))
    }
}

impl<'a> CommitType<'a> {
    pub fn new(name: &'a str, description: Option<&'a str>) -> Self {
        Self { name, description }
    }

    /// The length of the commit type name
    pub fn len(&self) -> usize {
        self.name.len()
    }
}
