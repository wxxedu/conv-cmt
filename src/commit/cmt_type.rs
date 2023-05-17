use std::fmt::Display;

use serde::{Deserialize, Serialize};

/// Represents a type of commit.
#[derive(Debug, Clone, PartialEq, Eq, Default, Deserialize, Serialize)]
pub struct CommitType {
    /// The name of the commit type
    pub name: String,
    /// The description of the commit type, to be shown in the terminal
    /// user interface
    pub description: Option<String>,
}

impl Display for CommitType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}: {}",
            self.name,
            self.description.clone().unwrap_or("".to_string())
        )
    }
}

impl CommitType {
    pub fn new<T: AsRef<str>, K: AsRef<str>>(
        name: T,
        description: Option<K>,
    ) -> Self {
        let name = name.as_ref().to_string();
        let description = description.map(|d| d.as_ref().to_string());
        Self { name, description }
    }

    /// The length of the commit type name
    pub fn len(&self) -> usize {
        self.name.len()
    }
}
