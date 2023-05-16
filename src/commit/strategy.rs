use std::fmt::Display;

use serde::{Deserialize, Serialize};

/// The strategy for the commit type
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Default, Deserialize, Serialize,
)]
pub enum CaseStrategy {
    /// The scope is all lowercase
    #[default]
    Lowercase,
    /// The scope is all uppercase
    Uppercase,
    /// The scope is capitalized
    Capitalized,
    /// The scope is unchanged
    Unchanged,
}

impl Display for CaseStrategy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            CaseStrategy::Lowercase => write!(f, "lowercase"),
            CaseStrategy::Uppercase => write!(f, "uppercase"),
            CaseStrategy::Capitalized => write!(f, "capitalized"),
            CaseStrategy::Unchanged => write!(f, "unchanged"),
        }
    }
}

impl CaseStrategy {
    /// Apply the case strategy to the scope
    pub fn apply<T: AsRef<str>>(&self, scope: T) -> String {
        match *self {
            CaseStrategy::Lowercase => scope.as_ref().to_lowercase(),
            CaseStrategy::Uppercase => scope.as_ref().to_uppercase(),
            CaseStrategy::Capitalized => {
                let mut scope = scope.as_ref().to_lowercase();
                scope[..1].make_ascii_uppercase();
                scope
            }
            CaseStrategy::Unchanged => scope.as_ref().to_string(),
        }
    }

    /// Verify that the scope matches the case strategy
    pub fn verify<T: AsRef<str>>(&self, scope: T) -> bool {
        match *self {
            CaseStrategy::Unchanged => true,
            CaseStrategy::Lowercase => {
                scope.as_ref().to_lowercase() == scope.as_ref()
            }
            CaseStrategy::Uppercase => {
                scope.as_ref().to_uppercase() == scope.as_ref()
            }
            CaseStrategy::Capitalized => {
                let mut scope = scope.as_ref().to_lowercase();
                scope[..1].make_ascii_uppercase();
                scope == scope.as_ref()
            }
        }
    }
}
