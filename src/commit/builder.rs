use super::{
    cmt_type::CommitType, commit::Commit, error::CommitError,
    strategy::CaseStrategy,
};

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct CommitBuilder<'a> {
    commit_type: Option<CommitType<'a>>,
    scope: Option<&'a str>,
    subject: Option<&'a str>,
    description: Option<&'a str>,
    is_breaking_change: bool,
    pub strategy: CaseStrategy,
}

impl<'a> CommitBuilder<'a> {
    /// Adds the commit type to the builder.
    pub fn commit_type(&mut self, commit_type: CommitType<'a>) -> &mut Self {
        self.commit_type = Some(commit_type);
        self
    }

    /// Adds the scope to the builder.
    pub fn scope(&mut self, scope: &'a str) -> Result<&mut Self, CommitError> {
        if !self.strategy.verify(scope) {
            return Err(CommitError::CaseError(
                scope.to_string(),
                self.strategy,
            ));
        }
        self.scope = Some(scope);
        Ok(self)
    }

    /// Adds the subject to the builder.
    pub fn subject(
        &mut self,
        subject: &'a str,
    ) -> Result<&mut Self, CommitError> {
        if !self.strategy.verify(subject) {
            return Err(CommitError::CaseError(
                subject.to_string(),
                self.strategy,
            ));
        }
        self.subject = Some(subject);
        Ok(self)
    }

    /// Adds the description to the builder.
    pub fn description(&mut self, description: &'a str) -> &mut Self {
        self.description = Some(description);
        self
    }

    /// Marks the commit as a breaking change.
    pub fn breaking_change(&mut self) -> &mut Self {
        self.is_breaking_change = true;
        self
    }

    /// Builds the commit.
    pub fn build(&self) -> Result<Commit<'a>, CommitError> {
        let commit_type = self
            .commit_type
            .ok_or(CommitError::MissingCommitTypeError)?;
        let subject = self.subject.ok_or(CommitError::MissingSubjectError)?;
        if commit_type.len() + subject.len() + self.scope.unwrap_or("").len()
            > 72
        {
            return Err(CommitError::SubjectTooLongError(
                commit_type.len()
                    + subject.len()
                    + self.scope.unwrap_or("").len(),
            ));
        }
        Ok(Commit {
            commit_type,
            scope: self.scope,
            subject,
            description: self.description,
            is_breaking_change: self.is_breaking_change,
        })
    }
}
