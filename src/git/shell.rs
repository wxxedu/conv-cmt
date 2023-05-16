use std::process::Command;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Shell;

impl Shell {
    pub fn new_command() -> Command {
        if cfg!(target_os = "windows") {
            let mut cmd = Command::new("cmd");
            cmd.arg("/C");
            cmd
        } else {
            let mut cmd = Command::new("sh");
            cmd.arg("-c");
            cmd
        }
    }

    pub fn new_git_command() -> Command {
        let mut cmd = Self::new_command();
        cmd.arg("git");
        cmd
    }
}
