use std::{error::Error, fmt::Display, process::Command};

#[derive(Debug)]
pub enum ShellError {
    ParseError(String),
    ExecError(String),
    CommandError(String),
}

impl Display for ShellError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ShellError::ParseError(msg) => {
                write!(f, "Error parsing shell output: {}", msg)
            }
            ShellError::ExecError(msg) => {
                write!(f, "Error executing shell command: {}", msg)
            }
            ShellError::CommandError(msg) => {
                write!(f, "Error executing shell command: {}", msg)
            }
        }
    }
}

impl Error for ShellError {}

pub trait Shell {
    fn run(command: impl AsRef<str>) -> Result<String, ShellError>;
}

#[derive(Debug)]
pub struct ShellImpl {}

impl Shell for ShellImpl {
    fn run(command: impl AsRef<str>) -> Result<String, ShellError> {
        dbg!(&command.as_ref());
        let output = Command::new("sh")
            .arg("-c")
            .arg(command.as_ref())
            .output()
            .map_err(|e| ShellError::ExecError(e.to_string()))?;
        dbg!(&output);
        if output.status.success() {
            Ok(String::from_utf8(output.stdout)
                .map_err(|e| ShellError::ParseError(e.to_string()))?)
        } else {
            Err(ShellError::CommandError(
                String::from_utf8(output.stderr)
                    .map_err(|e| ShellError::ParseError(e.to_string()))?,
            ))
        }
    }
}
