use std::{fmt::Display, process::Command};

use console::Term;

#[derive(Debug, Clone, PartialEq, Eq)]
enum CommitError {
    IndexOutOfRangeError { index: usize },
    IntParseError { input: String },
}

impl Display for CommitError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            CommitError::IndexOutOfRangeError { index } => {
                write!(f, "You entered an invalid index: {}, should be within range 1 - {}", index, CommitType::types().len())
            }
            CommitError::IntParseError { input } => {
                write!(
                    f,
                    "You entered an invalid index: {}, should be a number",
                    input
                )
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Commit<'a> {
    commit_type: CommitType,
    scope: Option<&'a str>,
    subject: &'a str,
}

impl<'a> Display for Commit<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let scope = match self.scope {
            Some(scope) => format!("({})", scope),
            None => String::new(),
        };
        write!(f, "{}{}: {}", self.commit_type, scope, self.subject)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
enum CommitType {
    Feat,
    Fix,
    Docs,
    Style,
    Refactor,
    Test,
    #[default]
    Chore,
}

impl Display for CommitType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            CommitType::Feat => write!(f, "feat"),
            CommitType::Fix => write!(f, "fix"),
            CommitType::Docs => write!(f, "docs"),
            CommitType::Style => write!(f, "style"),
            CommitType::Refactor => write!(f, "refactor"),
            CommitType::Test => write!(f, "test"),
            CommitType::Chore => write!(f, "chore"),
        }
    }
}

impl CommitType {
    pub fn describe(&self) -> &'static str {
        match *self {
            CommitType::Feat => "A new feature",
            CommitType::Fix => "A bug fix",
            CommitType::Docs => "Documentation only changes",
            CommitType::Style => {
                "Changes that do not affect the meaning of the code"
            }
            CommitType::Refactor => {
                "A code change that neither fixes a bug nor adds a feature"
            }
            CommitType::Test => {
                "Adding missing tests or correcting existing tests"
            }
            CommitType::Chore => {
                "Changes to the build process or auxiliary tools and libraries"
            }
        }
    }

    pub fn types() -> Vec<CommitType> {
        vec![
            CommitType::Feat,
            CommitType::Fix,
            CommitType::Docs,
            CommitType::Style,
            CommitType::Refactor,
            CommitType::Test,
            CommitType::Chore,
        ]
    }

    pub fn from_index_str(index: &str) -> Result<CommitType, CommitError> {
        // string to int
        if let Ok(index) = index.parse::<usize>() {
            return CommitType::from_index(index);
        }
        Err(CommitError::IntParseError {
            input: index.to_string(),
        })
    }

    pub fn from_index(index: usize) -> Result<CommitType, CommitError> {
        let types = CommitType::types();
        if index > 0 && index <= types.len() {
            return Ok(types[index - 1]);
        }
        Err(CommitError::IndexOutOfRangeError { index })
    }
}

fn prompt_commit_type(term: &Term) {
    for (i, commit_type) in CommitType::types().iter().enumerate() {
        term.write_line(&format!("{}. {}", i + 1, commit_type))
            .unwrap();
        term.write_line(&format!("   {}", commit_type.describe()))
            .unwrap();
    }
}

fn prompt_enter_commit_type(term: &Term) {
    term.write_line("Enter the index of the commit type:")
        .unwrap();
}

fn get_commit_type(term: &Term) -> CommitType {
    let mut input = term.read_line().unwrap();
    let mut commit_res = CommitType::from_index_str(&input);
    while let Err(err) = commit_res.clone() {
        term.write_line(&format!("{}", err)).unwrap();
        input = term.read_line().unwrap();
        commit_res = CommitType::from_index_str(&input);
    }
    commit_res.unwrap()
}

fn prompt_commit_scope(term: &Term) {
    term.write_line("Enter the scope of the commit: (press enter to continue with no scope)")
        .unwrap();
}

fn get_commit_scope(term: &Term) -> Option<String> {
    let input = term.read_line().unwrap();
    if input.is_empty() {
        return None;
    }
    Some(input)
}

fn prompt_commit_subject(term: &Term) {
    term.write_line("Enter the subject of the commit:").unwrap();
}

fn get_commit_subject(term: &Term) -> String {
    let mut res = term.read_line().unwrap();
    while res.is_empty() {
        term.write_line(
            "Subject cannot be empty, please re-enter a valid subject.",
        )
        .unwrap();
        res = term.read_line().unwrap();
    }
    // un-capitalise first letter
    let mut chars = res.chars();
    if let Some(first) = chars.next() {
        res = first.to_lowercase().to_string() + chars.as_str();
    }
    res
}

fn main() {
    let term = Term::stdout();
    prompt_commit_type(&term);
    prompt_enter_commit_type(&term);
    let commit_type = get_commit_type(&term);
    prompt_commit_scope(&term);
    let commit_scope = get_commit_scope(&term);
    prompt_commit_subject(&term);
    let commit_subject = get_commit_subject(&term);
    let commit = Commit {
        commit_type,
        scope: commit_scope.as_deref(),
        subject: &commit_subject,
    };
    let command_str = format!("git commit -m \"{}\"", commit);
    let command = Command::new("sh")
        .arg("-c")
        .arg(command_str)
        .spawn()
        .expect("failed to execute process");
    let output = command.wait_with_output().unwrap();
    if output.status.success() {
        term.write_line("Commit successful!").unwrap();
    } else {
        term.write_line("Commit failed!").unwrap();
    }
}
