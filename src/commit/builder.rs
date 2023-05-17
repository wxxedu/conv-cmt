use super::{
    cmt_type::CommitType,
    commit::Commit,
    error::{CommitComponent, CommitError},
    strategy::CaseStrategy,
};

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct CommitBuilder {
    commit_type: Option<CommitType>,
    scope: Option<String>,
    subject: Option<String>,
    description: Option<String>,
    is_breaking_change: bool,
    pub strategy: CaseStrategy,
}

impl CommitBuilder {
    /// Adds the commit type to the builder.
    pub fn commit_type(&mut self, commit_type: CommitType) -> &mut Self {
        self.commit_type = Some(commit_type);
        self
    }

    /// Adds the scope to the builder.
    pub fn scope(
        &mut self,
        scope: impl AsRef<str>,
    ) -> Result<&mut Self, CommitError> {
        let str_ref = scope.as_ref();
        if !self.strategy.verify(str_ref) {
            return Err(CommitError::CaseError(
                CommitComponent::Scope,
                str_ref.to_string(),
                self.strategy,
            ));
        }
        self.scope = Some(scope.as_ref().to_string());
        Ok(self)
    }

    /// Adds the subject to the builder.
    pub fn subject(
        &mut self,
        subject: impl AsRef<str>,
    ) -> Result<&mut Self, CommitError> {
        let str_ref = subject.as_ref();
        if !self.strategy.verify(str_ref) {
            return Err(CommitError::CaseError(
                CommitComponent::Subject,
                str_ref.to_string(),
                self.strategy,
            ));
        }
        self.subject = Some(subject.as_ref().to_string());
        Ok(self)
    }

    /// Adds the description to the builder.
    pub fn description(&mut self, description: impl AsRef<str>) -> &mut Self {
        self.description = Some(description.as_ref().to_string());
        self
    }

    /// Marks the commit as a breaking change.
    pub fn breaking_change(&mut self) -> &mut Self {
        self.is_breaking_change = true;
        self
    }

    /// Builds the commit.
    pub fn build(&self) -> Result<Commit, CommitError> {
        let commit_type = self
            .commit_type
            .clone()
            .ok_or(CommitError::MissingCommitTypeError)?;
        let subject = self
            .subject
            .clone()
            .ok_or(CommitError::MissingSubjectError)?;
        let scope_len = &self.scope.clone().unwrap_or("".to_string()).len();
        if commit_type.len() + subject.len() + scope_len > 72 {
            return Err(CommitError::SubjectTooLongError(
                commit_type.len() + subject.len() + scope_len,
            ));
        }
        Ok(Commit {
            commit_type,
            scope: self.scope.clone(),
            subject,
            description: self.description.clone(),
            is_breaking_change: self.is_breaking_change,
        })
    }
}
