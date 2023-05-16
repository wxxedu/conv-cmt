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
        let out_str = String::from_utf8(output.stdout).unwrap();
        println!("{}", out_str);
        !out_str.is_empty()
    }
}
