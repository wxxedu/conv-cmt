use super::shell::Shell;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Git;

impl Git {
    pub fn has_changes() -> bool {
        let output = Shell::new_git_command()
            .arg("status")
            .arg("--porcelain")
            .output()
            .expect("Failed to execute git status");

        !output.stdout.is_empty()
    }
}
