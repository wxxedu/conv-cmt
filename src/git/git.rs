use std::process::Command;

use crate::commit::commit::Commit;

use super::git_change::{GitChange, GitChangeStatus};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Git;

impl Git {
    pub fn new_git_command() -> Command {
        Command::new("git")
    }

    pub fn changes() -> Vec<GitChange> {
        let output = Self::new_git_command()
            .arg("status")
            .arg("--porcelain")
            .output()
            .expect("Failed to execute git status");
        let out_str = String::from_utf8(output.stdout).unwrap();
        let mut changes = Vec::new();
        for line in out_str.lines() {
            let mut chars = line.chars().collect::<Vec<char>>();
            chars.reverse();
            let status = match chars.pop() {
                Some(' ') => GitChangeStatus::Unstaged,
                _ => GitChangeStatus::Staged,
            };
            chars.pop();
            chars.reverse();
            let path = chars.iter().collect::<String>();
            // strip white space
            let path = path.trim();
            changes.push(GitChange::new(path.to_string(), status));
        }
        changes.sort_by(|a, b| a.partial_cmp(b).unwrap());
        changes
    }

    pub fn push() -> Result<String, String> {
        let output = Self::new_git_command()
            .arg("push")
            .output()
            .expect("Failed to execute git push");
        if output.status.success() {
            Ok(String::from_utf8(output.stdout).unwrap())
        } else {
            Err(String::from_utf8(output.stderr).unwrap())
        }
    }

    pub fn commit(cmt: &Commit) -> Result<String, String> {
        let output = Self::new_git_command()
            .arg("commit")
            .arg("-m")
            .arg(&cmt.to_string())
            .output()
            .expect("Failed to execute git commit");
        if output.status.success() {
            Ok(String::from_utf8(output.stdout).unwrap())
        } else {
            Err(String::from_utf8(output.stderr).unwrap())
        }
    }
}
