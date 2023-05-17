use dialoguer::{theme::ColorfulTheme, FuzzySelect};

use crate::commit::cmt_type::CommitType;

pub fn ask_commit_type<'a>(types: &Vec<CommitType<'a>>) -> CommitType<'a> {
    let selected = FuzzySelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Commit type")
        .default(0)
        .items(&types)
        .interact()
        .unwrap();
    types[selected].clone()
}

pub fn ask_scope() -> Option<String> {
    let res = dialoguer::Input::<String>::with_theme(&ColorfulTheme::default())
        .with_prompt("Scope")
        .allow_empty(true)
        .interact();
    match res {
        Ok(scope) => {
            if scope.is_empty() {
                None
            } else {
                Some(scope)
            }
        }
        Err(_) => None,
    }
}

pub fn ask_subject() -> String {
    dialoguer::Input::<String>::with_theme(&ColorfulTheme::default())
        .with_prompt("Subject")
        .interact()
        .unwrap()
}

pub fn ask_description() -> String {
    let should_add_description =
        dialoguer::Confirm::with_theme(&ColorfulTheme::default())
            .with_prompt("Add a description?")
            .interact()
            .unwrap();
    if should_add_description {
        let res = dialoguer::Editor::new().edit("").unwrap();
        match res {
            Some(description) => description,
            None => String::new(),
        }
    } else {
        String::new()
    }
}

pub fn ask_breaking_change() -> bool {
    dialoguer::Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Is this a breaking change?")
        .interact()
        .unwrap()
}
