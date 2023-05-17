use super::{
    cmt_type::CommitType,
    commit::Commit,
    constants::MAX_MESSAGE_LEN,
    error::{CasedComponent, CommitError},
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
    pub fn commit_type(
        &mut self,
        commit_type: CommitType,
    ) -> Result<&mut Self, CommitError> {
        self.commit_type = Some(commit_type);
        if self.message_len() + self.prefix_len() > MAX_MESSAGE_LEN {
            return Err(CommitError::SubjectTooLongError {
                available: MAX_MESSAGE_LEN - self.prefix_len(),
                actual: self.message_len(),
            });
        }
        Ok(self)
    }

    /// Adds the scope to the builder.
    pub fn scope(
        &mut self,
        scope: impl AsRef<str>,
    ) -> Result<&mut Self, CommitError> {
        let str_ref = scope.as_ref();
        if !self.strategy.verify(str_ref) {
            return Err(CommitError::CaseError(
                CasedComponent::Scope,
                str_ref.to_string(),
                self.strategy,
            ));
        }
        self.scope = Some(scope.as_ref().to_string());
        if self.message_len() + self.prefix_len() > MAX_MESSAGE_LEN {
            return Err(CommitError::SubjectTooLongError {
                available: MAX_MESSAGE_LEN - self.prefix_len(),
                actual: self.message_len(),
            });
        }
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
                CasedComponent::Subject,
                str_ref.to_string(),
                self.strategy,
            ));
        }
        if subject.as_ref().len() > MAX_MESSAGE_LEN - self.prefix_len() {
            return Err(CommitError::SubjectTooLongError {
                available: MAX_MESSAGE_LEN - self.prefix_len(),
                actual: subject.as_ref().len(),
            });
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
    pub fn breaking_change(&mut self) -> Result<&mut Self, CommitError> {
        self.is_breaking_change = true;
        if self.message_len() + self.prefix_len() > MAX_MESSAGE_LEN {
            return Err(CommitError::SubjectTooLongError {
                available: MAX_MESSAGE_LEN - self.prefix_len(),
                actual: self.message_len(),
            });
        }
        Ok(self)
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
        if self.message_len() + self.prefix_len() > MAX_MESSAGE_LEN {
            return Err(CommitError::SubjectTooLongError {
                available: MAX_MESSAGE_LEN - self.prefix_len(),
                actual: self.message_len(),
            });
        }
        Ok(Commit {
            commit_type,
            scope: self.scope.clone(),
            subject,
            description: self.description.clone(),
            is_breaking_change: self.is_breaking_change,
        })
    }

    pub fn prefix_len(&self) -> usize {
        let mut len: usize = 0;
        match &self.commit_type {
            Some(commit_type) => len += commit_type.len(),
            None => len += 0,
        }
        match &self.scope {
            Some(scope) => len += scope.len() + 2,
            None => len += 0,
        }
        if self.is_breaking_change {
            len += 1;
        }
        len
    }

    pub fn message_len(&self) -> usize {
        match &self.subject {
            Some(subject) => subject.len(),
            None => 0,
        }
    }
}
