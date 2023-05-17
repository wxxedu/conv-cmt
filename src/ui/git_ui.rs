use std::{ffi::OsStr, io::Write};

use console::{style, Term};
use dialoguer::{
    theme::Theme, Confirm, Editor, FuzzySelect, Input, MultiSelect,
};

use crate::{
    commit::{
        builder::CommitBuilder,
        cmt_type::CommitType,
        commit::Commit,
        error::{CasedComponent, CommitError},
        strategy::CaseStrategy,
    },
    git::{
        git::Git,
        git_change::{GitChange, GitChangeStatus, GitChanges},
    },
};

#[derive(Debug)]
pub struct GitUI<'a, T: AsRef<OsStr>, K: Theme> {
    term: &'a mut Term,
    theme: K,
    types: &'a Vec<CommitType>,
    builder: CommitBuilder,
    changes: Vec<GitChange>,
    scope: String,
    subject: String,
    description: String,
    editor: T,
}

impl<'a, T: AsRef<OsStr>, K: Theme> GitUI<'a, T, K> {
    pub fn new(
        term: &'a mut Term,
        theme: K,
        types: &'a Vec<CommitType>,
        strategy: CaseStrategy,
        editor: T,
    ) -> Self {
        Self {
            term,
            theme,
            builder: Commit::builder(strategy),
            changes: Vec::new(),
            types,
            editor,
            scope: String::new(),
            subject: String::new(),
            description: String::new(),
        }
    }

    pub fn show(&'a mut self) {
        self.ask_stage();
        self.ask_commit_type();
        self.ask_scope();
        self.ask_subject();
        self.ask_description();
        self.ask_breaking_change();
        self.ask_review_commit();
        self.ask_push();
    }

    fn ask_stage(&mut self) {
        self.changes = Git::changes();
        let mut checked = vec![false; self.changes.len()];
        for (i, change) in self.changes.iter().enumerate() {
            if change.status == GitChangeStatus::Staged {
                checked[i] = true;
            }
        }
        let selected =
        MultiSelect::with_theme(&self.theme)
            .with_prompt("Select changes to stage (press space to select/deselect, a to toggle all, and enter to continue)")
            .items(&self.changes)
            .defaults(&checked)
            .interact()
            .unwrap();
        for i in 0..self.changes.len() {
            if selected.contains(&i) {
                self.changes[i].stage();
            } else {
                self.changes[i].unstage();
            }
        }
        if !self.changes.has_staged_changes() {
            let res = FuzzySelect::with_theme(&self.theme)
                .with_prompt(
                    "No staged changes found. Do you want to quit or retry?",
                )
                .items(&["Quit", "Retry"])
                .interact()
                .unwrap();
            match res {
                0 => std::process::exit(0),
                1 => self.ask_stage(),
                _ => {}
            }
        }
    }

    fn ask_commit_type(&mut self) {
        let selected = FuzzySelect::with_theme(&self.theme)
            .with_prompt("Commit type")
            .default(0)
            .items(self.types)
            .interact()
            .unwrap();
        let res = self.builder.commit_type(self.types[selected].clone());
        match res {
            Ok(_) => {}
            Err(error) => {
                self.handle_commit_error(error);
            }
        }
    }

    fn ask_scope(&mut self) {
        let res = Input::<String>::with_theme(&self.theme)
            .with_prompt("Scope")
            .allow_empty(true)
            .interact();
        match res {
            Ok(scp) => {
                if !scp.is_empty() {
                    self.scope.clear();
                    self.scope.push_str(&self.builder.strategy.apply(&scp));
                    let tmp = self.builder.scope(&self.scope);
                    match tmp {
                        Ok(_) => {}
                        Err(error) => {
                            self.handle_commit_error(error);
                        }
                    }
                }
            }
            Err(_) => {}
        }
    }

    fn ask_subject(&mut self) {
        let res = Input::<String>::with_theme(&self.theme)
            .with_prompt("Subject")
            .interact()
            .unwrap();
        if !res.is_empty() {
            self.subject.clear();
            self.subject.push_str(&self.builder.strategy.apply(&res));
            let tmp = self.builder.subject(&self.subject);
            match tmp {
                Ok(_) => {}
                Err(error) => {
                    self.handle_commit_error(error);
                }
            }
        }
    }

    fn ask_description(&mut self) {
        let should_add_description = Confirm::with_theme(&self.theme)
            .with_prompt("Add a description?")
            .interact()
            .unwrap();
        if should_add_description {
            let res = Editor::new().executable(&self.editor).edit("").unwrap();
            match res {
                Some(description) => {
                    self.description.clear();
                    self.description.push_str(&description);
                    self.builder.description(&self.description);
                }
                None => {}
            }
        }
    }

    fn ask_breaking_change(&mut self) {
        let change_is_safe = dialoguer::Confirm::with_theme(&self.theme)
        .with_prompt(
            "Is this a safe change? (answer no if this is a breaking change)",
        )
        .interact()
        .unwrap();
        if !change_is_safe {
            let res = self.builder.breaking_change();
            match res {
                Ok(_) => {}
                Err(error) => {
                    self.handle_commit_error(error);
                }
            }
        }
    }

    fn get_commit(&mut self) -> Commit {
        let res = self.builder.build();
        match res {
            Ok(commit) => commit,
            Err(error) => {
                self.handle_commit_error(error);
                self.get_commit()
            }
        }
    }

    fn handle_commit_error(&mut self, error: CommitError) {
        self.term.write_line(&format!("{}", error)).unwrap();
        match error {
            CommitError::SubjectTooLongError {
                available: _,
                actual: _,
            } => {
                self.ask_subject();
            }
            CommitError::MissingCommitTypeError => {
                self.ask_commit_type();
            }
            CommitError::MissingSubjectError => {
                self.ask_subject();
            }
            CommitError::CaseError(component, _, _) => match component {
                CasedComponent::Subject => {
                    self.ask_subject();
                }
                CasedComponent::Scope => {
                    self.ask_scope();
                }
            },
        }
    }

    fn ask_review_commit(&mut self) {
        let commit = self.get_commit();
        let revise_options = vec![
            "Confirm",
            "Commit Type",
            "Scope",
            "Subject",
            "Description",
            "Breaking Change",
            "Quit",
        ];
        let res = FuzzySelect::with_theme(&self.theme)
            .with_prompt(format!(
                "Review commit: {}",
                style(&commit).cyan().bold()
            ))
            .default(0)
            .items(&revise_options)
            .interact()
            .unwrap();
        match res {
            0 => {
                let res = Git::commit(&commit);
                match res {
                    Ok(msg) => {
                        self.term.write(&msg).unwrap();
                    }
                    Err(error) => {
                        self.term.write(&error).unwrap();
                        self.ask_review_commit();
                    }
                }
            }
            1 => {
                self.ask_commit_type();
                self.ask_review_commit();
            }
            2 => {
                self.ask_scope();
                self.ask_review_commit();
            }
            3 => {
                self.ask_subject();
                self.ask_review_commit();
            }
            4 => {
                self.ask_description();
                self.ask_review_commit();
            }
            5 => {
                self.ask_breaking_change();
                self.ask_review_commit();
            }
            6 => {
                let res = Confirm::with_theme(&self.theme)
                    .with_prompt("Are you sure you want to quit?")
                    .interact()
                    .unwrap();
                if res {
                    std::process::exit(0);
                } else {
                    self.ask_review_commit();
                }
            }
            _ => {}
        }
    }

    fn ask_push(&mut self) {
        let res = Confirm::with_theme(&self.theme)
            .with_prompt("Do you want to push?")
            .interact()
            .unwrap();
        if res {
            let res = Git::push();
            match res {
                Ok(msg) => {
                    self.term.write(&msg).unwrap();
                }
                Err(error) => {
                    self.term.write(&error).unwrap();
                    self.ask_push();
                }
            }
        }
    }
}
