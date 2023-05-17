use std::fmt::Display;

use super::{
    builder::CommitBuilder, cmt_type::CommitType, strategy::CaseStrategy,
};

/// Represents a commit.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Commit {
    pub commit_type: CommitType,
    pub scope: Option<String>,
    pub subject: String,
    pub description: Option<String>,
    pub is_breaking_change: bool,
}

impl Commit {
    /// Creates a builder that can be used to build a new commit.
    pub fn builder(strategy: CaseStrategy) -> CommitBuilder {
        let mut builder = CommitBuilder::default();
        builder.strategy = strategy;
        builder
    }
}

impl Display for Commit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let scope = match &self.scope {
            Some(scope) => format!("({})", scope),
            None => String::new(),
        };
        let description = match &self.description {
            Some(description) => {
                if description.is_empty() {
                    String::new()
                } else {
                    format!("\n\n{}", description)
                }
            }
            None => String::new(),
        };
        let breaking = if self.is_breaking_change { "!" } else { "" };
        let content = format!(
            "{}{}{}: {}{}",
            self.commit_type.name, scope, breaking, self.subject, description
        );

        // replace the unescaped " with \"
        // this probably would be better outside of the display impl, as it
        // would make it such that every time the commit is displayed, it
        // would run once, which isn't very efficient. Regardless, i will
        // leave it here for now.
        // let pattern = Regex::new(r#"(?<!\\)""#).unwrap();
        // let content = pattern.replace_all(&content, r#"\""#);
        write!(f, "{}", content)
    }
}
