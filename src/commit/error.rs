use std::{error::Error, fmt::Display};

use crate::commit::constants::SUBJECT_MAX_LEN;

use super::strategy::CaseStrategy;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CommitError {
    SubjectTooLongError(usize),
    MissingCommitTypeError,
    MissingSubjectError,
    CaseError(CommitComponent, String, CaseStrategy),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CommitComponent {
    CommitType,
    Scope,
    Subject,
    Description,
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

impl Display for CommitComponent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CommitComponent::CommitType => write!(f, "Commit Type"),
            CommitComponent::Scope => write!(f, "Scope"),
            CommitComponent::Subject => write!(f, "Subject"),
            CommitComponent::Description => write!(f, "Description"),
        }
    }
}

impl Error for CommitError {}
