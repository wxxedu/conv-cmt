use std::{error::Error, fmt::Display};

use crate::config::config::Config;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CommitType<'a> {
    pub name: &'a str,
    pub description: &'a str,
}

impl<'a> CommitType<'a> {
    pub fn new(name: &'a str, description: &'a str) -> Self {
        Self { name, description }
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
pub enum CaseResolutionStrategy {
    #[default]
    None,
    LowerCase,
    UpperCase,
    Capitalized,
}

impl CaseResolutionStrategy {
    pub fn resolve(&self, string: impl AsRef<str>) -> String {
        let string = string.as_ref();
        match self {
            Self::None => string.to_string(),
            Self::LowerCase => string.to_lowercase(),
            Self::UpperCase => string.to_uppercase(),
            Self::Capitalized => {
                let mut chars = string.chars();
                match chars.next() {
                    None => String::new(),
                    Some(c) => {
                        c.to_uppercase().collect::<String>() + chars.as_str()
                    }
                }
            }
        }
    }
}

impl Display for CaseResolutionStrategy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::None => write!(f, "none"),
            Self::LowerCase => write!(f, "lowercase"),
            Self::UpperCase => write!(f, "uppercase"),
            Self::Capitalized => write!(f, "capitalized"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Commit<'a, T: Config<'a>> {
    pub config: &'a T,
    pub commit_type: CommitType<'a>,
    pub scope: Option<String>,
    pub is_breaking_change: bool,
    pub description: String,
    pub body: Option<String>,
    pub footer: Option<String>,
}

impl<'a, T: Config<'a>> Commit<'a, T> {
    pub fn builder(config: &'a T) -> CommitBuilder<'a, T> {
        CommitBuilder::new(config)
    }
}

impl<'a, T: Config<'a>> Display for Commit<'a, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.commit_type.name)?;
        if let Some(scope) = &self.scope {
            write!(f, "({})", scope)?;
        }
        if self.is_breaking_change {
            write!(f, "!")?;
        }
        write!(f, ": {}", self.description)?;
        if let Some(body) = &self.body {
            write!(f, "\n\n{}", body)?;
        }
        if let Some(footer) = &self.footer {
            if self.body.is_none() {
                write!(f, "\n\n")?;
            }
            write!(f, "\n\n{}", footer)?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CommitBuildError {
    MissingCommitType,
    MissingDescription,
    ExceedsMaxLineLength {
        type_len: usize,
        scope_len: usize,
        breaking_change_len: usize,
        description_len: usize,
        max_len: usize,
    },
    ScopeLengthExceedsMax {
        scope_len: usize,
        max_len: usize,
    },
}

impl Display for CommitBuildError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CommitBuildError::MissingCommitType => {
                write!(f, "missing commit type")
            }
            CommitBuildError::MissingDescription => {
                write!(f, "missing description")
            }
            CommitBuildError::ExceedsMaxLineLength {
                type_len,
                scope_len,
                description_len,
                breaking_change_len,
                max_len,
            } => write!(
                f,
                "exceeds max line length (type: {} + scope: {} + breaking: {} + description: {} = {} > {})",
                type_len, scope_len, breaking_change_len, description_len, type_len + scope_len + breaking_change_len + description_len, max_len
            ),
            CommitBuildError::ScopeLengthExceedsMax { scope_len, max_len } => {
                write!(f, "scope length exceeds max ({} > {})", scope_len, max_len)
            }
        }
    }
}

impl Error for CommitBuildError {}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CommitBuilder<'a, T: Config<'a>> {
    config: &'a T,
    commit_type: CommitType<'a>,
    scope: Option<String>,
    is_breaking_change: bool,
    description: String,
    body: Option<String>,
    footer: Option<String>,
}

impl<'a, T: Config<'a>> CommitBuilder<'a, T> {
    pub fn new(config: &'a T) -> Self {
        Self {
            config,
            commit_type: CommitType::new("feat", "A new feature"),
            scope: None,
            is_breaking_change: false,
            description: String::new(),
            body: None,
            footer: None,
        }
    }

    fn commit_type_len(&self) -> usize {
        self.commit_type.name.len()
    }

    fn scope_len(&self) -> usize {
        self.scope.as_ref().map(|s| s.len() + 2).unwrap_or(0)
    }

    fn is_breaking_change_len(&self) -> usize {
        if self.is_breaking_change {
            1
        } else {
            0
        }
    }

    fn description_len(&self) -> usize {
        self.description.len()
    }

    pub fn commit_type(
        &mut self,
        commit_type: CommitType<'a>,
    ) -> Result<&mut Self, CommitBuildError> {
        if commit_type.name.len()
            + self.scope_len()
            + self.is_breaking_change_len()
            + self.description_len()
            > self.config.max_commit_message_length()
        {
            return Err(CommitBuildError::ExceedsMaxLineLength {
                type_len: commit_type.name.len(),
                scope_len: self.scope_len(),
                breaking_change_len: self.is_breaking_change_len(),
                description_len: self.description_len(),
                max_len: self.config.max_commit_message_length(),
            });
        }
        self.commit_type = commit_type;
        Ok(self)
    }

    pub fn scope(
        &mut self,
        scope: impl AsRef<str>,
    ) -> Result<&mut Self, CommitBuildError> {
        let scope = scope.as_ref();
        if let Some(scope_len) = self.config.max_scope_length() {
            if scope.len() > scope_len {
                return Err(CommitBuildError::ScopeLengthExceedsMax {
                    scope_len,
                    max_len: self.config.max_scope_length().unwrap(),
                });
            }
        }
        if self.commit_type_len()
            + scope.len()
            + self.is_breaking_change_len()
            + self.description_len()
            > self.config.max_commit_message_length()
        {
            return Err(CommitBuildError::ExceedsMaxLineLength {
                type_len: self.commit_type.name.len(),
                scope_len: scope.len(),
                breaking_change_len: self.is_breaking_change_len(),
                description_len: self.description_len(),
                max_len: self.config.max_commit_message_length(),
            });
        }
        self.scope = Some(scope.to_string());
        Ok(self)
    }

    pub fn is_breaking_change(
        &mut self,
        is_breaking_change: bool,
    ) -> Result<&mut Self, CommitBuildError> {
        let breaking_change_len = if is_breaking_change { 1 } else { 0 };
        if self.commit_type_len()
            + self.scope_len()
            + breaking_change_len
            + self.description_len()
            > self.config.max_commit_message_length()
        {
            return Err(CommitBuildError::ExceedsMaxLineLength {
                type_len: self.commit_type.name.len(),
                scope_len: self.scope_len(),
                breaking_change_len: self.is_breaking_change_len(),
                description_len: self.description_len(),
                max_len: self.config.max_commit_message_length(),
            });
        }
        self.is_breaking_change = is_breaking_change;
        Ok(self)
    }

    pub fn description(
        &mut self,
        description: impl AsRef<str>,
    ) -> Result<&mut Self, CommitBuildError> {
        let description = description.as_ref();
        if self.commit_type_len()
            + self.scope_len()
            + self.is_breaking_change_len()
            + description.len()
            > self.config.max_commit_message_length()
        {
            return Err(CommitBuildError::ExceedsMaxLineLength {
                type_len: self.commit_type.name.len(),
                scope_len: self.scope_len(),
                breaking_change_len: self.is_breaking_change_len(),
                description_len: description.len(),
                max_len: self.config.max_commit_message_length(),
            });
        }
        self.description = description.to_string();
        Ok(self)
    }

    pub fn body(&mut self, body: impl Into<String>) -> &mut Self {
        let body = body.into();
        if body.len() == 0 || body.chars().all(|c| c.is_whitespace()) {
            return self;
        }
        self.body = Some(body);
        self
    }

    pub fn footer(&mut self, footer: impl Into<String>) -> &mut Self {
        let footer = footer.into();
        if footer.len() == 0 || footer.chars().all(|c| c.is_whitespace()) {
            return self;
        }
        self.footer = Some(footer);
        self
    }

    pub fn build(&self) -> Commit<'a, T> {
        Commit {
            config: self.config,
            commit_type: self.commit_type.clone(),
            scope: self.scope.clone(),
            is_breaking_change: self.is_breaking_change,
            description: self.description.clone(),
            body: self.body.clone(),
            footer: self.footer.clone(),
        }
    }
}
