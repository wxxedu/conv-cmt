use super::{commit_scope::CommitScopeFL, commit_type::CommitType};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Commit<'a, const SCOPE_MAX_LEN: usize> {
    commit_type: &'a CommitType<'a>,
    scope: CommitScopeFL<SCOPE_MAX_LEN>,
    subject: &'a str,
    description: Option<&'a str>,
}
