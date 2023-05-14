use std::{fmt::Display, io::Write, process::Command};

use console::{style, Term};

macro_rules! println_cyan {
    ($term: expr, $($arg:expr),*) => {
        $term.write_line(&format!("{}", style(format!($($arg,)*)).cyan())).unwrap();
    };
}

macro_rules! print_cyan {
    ($term: expr, $($arg:expr),*) => {
        $term.write(&format!("{}", style(format!($($arg,)*)).cyan()).as_bytes()).unwrap();
    };
}

macro_rules! println_red {
    ($term: expr, $($arg:expr),*) => {
        $term.write_line(&format!("{}", style(format!($($arg,)*)).red())).unwrap();
    };
}

// macro_rules! print_red {
//     ($term: expr, $($arg:expr),*) => {
//         $term.write(&format!("{}", style(format!($($arg,)*)).red()).as_bytes()).unwrap();
//     };
// }

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

fn get_commit_type(term: &mut Term) -> CommitType {
    for (i, commit_type) in CommitType::types().iter().enumerate() {
        println_cyan!(
            term,
            "{}. {}: {}",
            i + 1,
            style(commit_type).bold(),
            style(commit_type.describe()).cyan().dim()
        );
    }
    print_cyan!(term, "{}", style("Enter the type of the commit: ").bold());

    let mut input = term.read_line().unwrap();
    let mut commit_res = CommitType::from_index_str(&input);
    while let Err(err) = commit_res.clone() {
        println_red!(term, "{}", err);
        input = term.read_line().unwrap();
        commit_res = CommitType::from_index_str(&input);
    }
    commit_res.unwrap()
}

fn get_commit_scope(term: &mut Term) -> Option<String> {
    print_cyan!(term, "{}", style("Enter the scope of the commit: ").bold());
    let input = term.read_line().unwrap();
    if input.is_empty() {
        return None;
    }
    // un-capitalise first letter
    let mut chars = input.chars();
    if let Some(first) = chars.next() {
        return Some(first.to_lowercase().to_string() + chars.as_str());
    }
    Some(input)
}

fn get_commit_subject(term: &mut Term) -> String {
    print_cyan!(
        term,
        "{}",
        style("Enter the subject of the commit: ").bold()
    );
    let mut res = term.read_line().unwrap();
    while res.is_empty() {
        println_red!(
            term,
            "Subject cannot be empty, please re-enter a valid subject."
        );
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
    let mut term = Term::stdout();
    let commit_type = get_commit_type(&mut term);
    let commit_scope = get_commit_scope(&mut term);
    let commit_subject = get_commit_subject(&mut term);
    let commit = Commit {
        commit_type,
        scope: commit_scope.as_deref(),
        subject: &commit_subject,
    };
    let command_str = format!("git commit -m \"{}\"", commit);
    let command = Command::new("sh")
        .arg("-c")
        .arg(command_str.clone())
        .spawn()
        .or_else(|_| Command::new("cmd").arg("/C").arg(command_str).spawn());
    if let Err(err) = command {
        println_red!(term, "{}", err);
        return;
    }

    let output = command.unwrap().wait_with_output().unwrap();
    if output.status.success() {
        term.write_line("Commit successful!").unwrap();
    } else {
        term.write_line("Commit failed!").unwrap();
    }
}
