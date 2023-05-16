use super::{
    builder::CommitBuilder, cmt_type::CommitType, strategy::CaseStrategy,
};

/// Represents a commit.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Commit<'a> {
    pub commit_type: CommitType<'a>,
    pub scope: Option<&'a str>,
    pub subject: &'a str,
    pub description: Option<&'a str>,
}

impl<'a> Commit<'a> {
    /// Creates a builder that can be used to build a new commit.
    pub fn builder(strategy: CaseStrategy) -> CommitBuilder<'a> {
        let mut builder = CommitBuilder::default();
        builder.strategy = strategy;
        builder
    }
}
