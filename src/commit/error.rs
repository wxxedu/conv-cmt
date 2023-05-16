use std::error::Error;

use crate::commit::constants::SUBJECT_MAX_LEN;

use super::strategy::CaseStrategy;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CommitError {
    SubjectTooLongError(usize),
    MissingCommitTypeError,
    MissingSubjectError,
    CaseError(String, CaseStrategy),
}

impl std::fmt::Display for CommitError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            CommitError::SubjectTooLongError(len) => {
                write!(
                    f, 
                    "The subject is too long: {}, should be less than {} characters", 
                    len, 
                    SUBJECT_MAX_LEN
                )
            }
            CommitError::MissingCommitTypeError => {
                write!(f, "You did not select a commit type")
            }
            CommitError::MissingSubjectError => {
                write!(f, "You did not enter a subject")
            }
            CommitError::CaseError(content, strategy) => {
                write!(
                    f, 
                    "The content '{}' does not match the case strategy: {}",
                    content,
                    strategy
                )
            }
        }
    }
}

impl Error for CommitError {}
