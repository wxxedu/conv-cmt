use std::{
    error::Error,
    fmt::{Debug, Display},
    marker::PhantomData,
    path::Path,
};

use crate::{
    config::config::Config,
    shell::shell::{Shell, ShellError},
};

use super::{commit::Commit, item::GitItem};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum GitError {
    GitError(String),
    NoGitRepository,
    NoRemote,
    NoUpstream,
    ParseError(String),
    ExecError(String),
}

impl Display for GitError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GitError::GitError(msg) => write!(f, "{}", msg),
            GitError::NoGitRepository => write!(f, "No git repository found"),
            GitError::NoRemote => write!(f, "No remote found"),
            GitError::NoUpstream => write!(f, "No upstream found"),
            GitError::ExecError(msg) => {
                write!(f, "Error executing git command: {}", msg)
            }
            GitError::ParseError(msg) => {
                write!(f, "Error parsing git output: {}", msg)
            }
        }
    }
}

impl Error for GitError {}

unsafe impl Send for GitError {}

unsafe impl Sync for GitError {}

pub trait Git {
    /// Gets all the items in the current git repository
    fn get_statuses_of_items(&self) -> Result<Vec<GitItem>, GitError>;

    /// Get status of the item specified by the given path
    fn get_item_status(
        &self,
        path: impl AsRef<Path>,
    ) -> Result<GitItem, GitError>;

    /// Adds the given [GitItem] to the git repository
    fn add(&self, item: &mut GitItem) -> Result<String, GitError>;

    /// Removes the given [GitItem] from the git repository
    fn reset(&self, item: &mut GitItem) -> Result<String, GitError>;

    /// Commits with the given [Commit] object
    fn commit<'a, T: Config<'a>>(
        &self,
        item: &Commit<'a, T>,
    ) -> Result<String, GitError>;

    /// Pushes the current branch to the remote
    fn push(&self) -> Result<String, GitError>;

    /// Pushes the current branch to remote, creating it if it doesn't exist
    fn push_with_upstream(&self) -> Result<String, GitError>;

    /// Gets the current branch name
    fn get_current_branch(&self) -> Result<String, GitError>;

    /// Init a git repository
    fn init(&self) -> Result<String, GitError>;
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct GitImpl<'a, SH: Shell> {
    pub root: &'a Path,
    _shell: PhantomData<SH>,
}

impl<'a, SH: Shell> GitImpl<'a, SH> {
    pub fn new(root: &'a Path) -> Self {
        Self {
            root,
            _shell: PhantomData,
        }
    }
}

impl<'b, SH> Git for GitImpl<'b, SH>
where
    SH: Shell + Debug,
{
    fn get_statuses_of_items(&self) -> Result<Vec<GitItem>, GitError> {
        let output = SH::run(format!(
            "cd {} && git status --porcelain",
            self.root.display()
        ))
        .map_err(|e| GitError::ExecError(e.to_string()))?;
        let mut items = Vec::new();
        for line in output.lines() {
            let item = GitItem::parse(line);
            match item {
                Some(item) => items.push(item),
                None => {
                    return Err(GitError::GitError(format!(
                        "Unknown git item: {}",
                        line
                    )))
                }
            }
        }
        Ok(items)
    }

    fn get_item_status(
        &self,
        path: impl AsRef<Path>,
    ) -> Result<GitItem, GitError> {
        let result = SH::run(format!(
            "cd {} && git status --porcelain {}",
            self.root.display(),
            path.as_ref().display()
        ))
        .map_err(|e| GitError::ExecError(e.to_string()))?;
        let mut lines = result.lines();
        let line = lines
            .next()
            .ok_or_else(|| GitError::ParseError("No output".to_string()))?;
        let item = GitItem::parse(line);
        match item {
            Some(item) => Ok(item),
            None => {
                Err(GitError::ParseError(format!("Unknown git item: {}", line)))
            }
        }
    }

    fn add(&self, item: &mut GitItem) -> Result<String, GitError> {
        let output = SH::run(format!(
            "cd {} && git add {}",
            self.root.display(),
            item.path.display()
        ));
        match output {
            Ok(output) => {
                let status = self.get_item_status(&item.path);
                match status {
                    Ok(updated) => {
                        item.copy_status_from(&updated);
                    }
                    Err(e) => return Err(e),
                }
                Ok(output)
            }
            Err(e) => Err(GitError::ExecError(e.to_string())),
        }
    }

    fn reset(&self, item: &mut GitItem) -> Result<String, GitError> {
        let output = SH::run(format!(
            "cd {} && git reset HEAD {}",
            self.root.display(),
            item.path.display()
        ));
        match output {
            Ok(output) => {
                let status = self.get_item_status(&item.path);
                match status {
                    Ok(updated) => {
                        item.copy_status_from(&updated);
                    }
                    Err(e) => return Err(e),
                }
                Ok(output)
            }
            Err(e) => Err(GitError::ExecError(e.to_string())),
        }
    }

    fn commit<'a, C: Config<'a>>(
        &self,
        item: &Commit<'a, C>,
    ) -> Result<String, GitError> {
        let output = SH::run(format!(
            "cd {} && git commit -m \"{}\"",
            self.root.display(),
            item
        ))
        .map_err(|e| GitError::ExecError(e.to_string()))?;
        Ok(output)
    }

    fn push(&self) -> Result<String, GitError> {
        let output = SH::run(format!("cd {} && git push", self.root.display()))
            .map_err(|e| GitError::ExecError(e.to_string()))?;
        Ok(output)
    }

    fn push_with_upstream(&self) -> Result<String, GitError> {
        let branch = self.get_current_branch()?;
        let output = SH::run(format!(
            "cd {} && git push --set-upstream origin {}",
            self.root.display(),
            branch
        ));
        match output {
            Ok(output) => Ok(output),
            Err(e) => Err(GitError::ExecError(e.to_string())),
        }
    }

    fn get_current_branch(&self) -> Result<String, GitError> {
        let output = SH::run(format!(
            "cd {} && git rev-parse --abbrev-ref HEAD",
            self.root.to_str().unwrap()
        ));
        dbg!(&output);
        match output {
            Ok(output) => Ok(output.trim().to_string()),
            Err(e) => Err(GitError::ExecError(e.to_string())),
        }
    }

    fn init(&self) -> Result<String, GitError> {
        let path = self
            .root
            .to_str()
            .ok_or_else(|| GitError::GitError("Invalid path".to_string()))?;
        let res = SH::run(format!("cd {} && git init", path));
        match res {
            Ok(output) => Ok(output),
            Err(e) => match e {
                ShellError::ParseError(error) => {
                    Err(GitError::ParseError(error.to_string()))
                }
                ShellError::ExecError(error) => {
                    Err(GitError::ExecError(error.to_string()))
                }
                ShellError::CommandError(error) => {
                    Err(GitError::GitError(error.to_string()))
                }
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use tempfile::{tempdir, TempDir};

    use crate::shell::shell::ShellImpl;

    use super::*;

    fn check_exists_git(path: impl AsRef<Path>) -> bool {
        let path = path.as_ref();
        let git_path = path.join(".git");
        git_path.is_dir()
    }

    fn init_dummy_git_repository() -> TempDir {
        let temp_dir = tempdir().unwrap();
        let res = ShellImpl::run(format!(
            "cd {} && git init && touch dummy && git add dummy && git commit -m 'dummy' && touch dummy2 && git add dummy2 && touch dummy3",
            temp_dir.path().to_str().unwrap()
        ));
        assert!(res.is_ok());
        temp_dir
    }

    #[test]
    fn test_init() {
        let temp_dir = tempdir().unwrap();
        let path = temp_dir.path().to_path_buf();
        let git = GitImpl::<ShellImpl>::new(&path);
        let result = git.init();
        assert!(result.is_ok());
        assert!(&path.exists());
        assert!(check_exists_git(&path));
    }

    #[test]
    fn test_get_current_branch() {
        let temp_dir = init_dummy_git_repository();
        let path = temp_dir.path().to_path_buf();
        let git = GitImpl::<ShellImpl>::new(&path);
        let result = git.get_current_branch();
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "main");
    }

    #[test]
    fn test_get_item_status() {
        let temp_dir = init_dummy_git_repository();
        let path = temp_dir.path().to_path_buf();
        let git = GitImpl::<ShellImpl>::new(&path);

        let result = git.get_item_status(&path.join("dummy"));
        assert!(result.is_err());

        let result = git.get_item_status(&path.join("dummy2"));
        assert!(result.is_ok());
        let status = result.unwrap();
        assert!(status.is_staged());

        let result = git.get_item_status(&path.join("dummy3"));
        assert!(result.is_ok());
        let status = result.unwrap();
        assert!(!status.is_staged());
    }
}
