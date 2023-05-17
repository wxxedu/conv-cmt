use std::{error::Error, fmt::Display};

use crate::commit::constants::MAX_MESSAGE_LEN;

use super::strategy::CaseStrategy;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CommitError {
    SubjectTooLongError{available: usize, actual: usize},
    MissingCommitTypeError,
    MissingSubjectError,
    CaseError(CasedComponent, String, CaseStrategy),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CasedComponent {
    Scope,
    Subject,
}

impl std::fmt::Display for CommitError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            CommitError::SubjectTooLongError{available, actual} => {
                write!(
                    f, 
                    "The subject is too long: {}, should be less than {} characters such that the length of the entire commit message is less than {} characters",
                    actual,
                    available,
                    MAX_MESSAGE_LEN
                )
            }
            CommitError::MissingCommitTypeError => {
                write!(f, "You did not select a commit type")
            }
            CommitError::MissingSubjectError => {
                write!(f, "You did not enter a subject")
            }
            CommitError::CaseError(component, content, strategy) => {
                write!(
                    f, 
                    "The content '{}' for {} does not match the case strategy: {}",
                    content,
                    component,
                    strategy
                )
            }
        }
    }
}

impl Display for CasedComponent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CasedComponent::Scope => write!(f, "Scope"),
            CasedComponent::Subject => write!(f, "Subject"),
        }
    }
}

impl Error for CommitError {}
