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
                Some('M') => GitChangeStatus::Staged,
                Some('?') => GitChangeStatus::Untracked,
                Some('A') => GitChangeStatus::Staged,
                _ => panic!("Unknown git status: {}", line),
            };
            chars.pop();
            chars.reverse();
            let path = chars.iter().collect::<String>();
            // strip white space
            let path = path.trim();
            changes.push(GitChange {
                path: path.to_string(),
                status,
            });
        }
        changes.sort_by(|a, b| a.partial_cmp(b).unwrap());
        changes
    }

    pub fn stage_all() {
        let output = Self::new_git_command()
            .arg("add")
            .arg(".")
            .output()
            .expect("Failed to execute git add");
        if !output.status.success() {
            panic!("Failed to execute git add");
        }
    }

    pub fn push() {
        let output = Self::new_git_command()
            .arg("push")
            .output()
            .expect("Failed to execute git push");
        if !output.status.success() {
            panic!("Failed to execute git push");
        } else {
            // print out
            println!("{}", String::from_utf8(output.stdout).unwrap());
        }
    }

    pub fn commit(cmt: &Commit) {
        let output = Self::new_git_command()
            .arg("commit")
            .arg("-m")
            .arg(&cmt.to_string())
            .output()
            .expect("Failed to execute git commit");
        if !output.status.success() {
            panic!("Failed to execute git commit");
        }
    }
}
