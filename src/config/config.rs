use crate::git::commit::{CaseResolutionStrategy, CommitType};

pub trait Config<'a> {
    /// The maximum length of a commit's title, which includes the commit type,
    /// the scope, the "!" for breaking changes, and the description.
    fn max_commit_message_length(&self) -> usize;

    /// The maximum length of a scope, if exists.
    fn max_scope_length(&self) -> Option<usize>;

    /// The available commit types.
    fn commit_types(&self) -> &'a [CommitType<'a>];

    /// The case resolution strategy for commit type.
    fn commit_type_strategy(&self) -> CaseResolutionStrategy;

    /// The case resolution strategy for scope.
    fn scope_strategy(&self) -> CaseResolutionStrategy;

    /// The case resolution strategy for description.
    fn description_strategy(&self) -> CaseResolutionStrategy;
}
