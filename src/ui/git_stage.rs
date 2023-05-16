use dialoguer::MultiSelect;

use crate::git::{
    git::Git,
    git_change::{GitChange, GitChangeStatus},
};

pub fn show_stage_all_or_select_view(changes: &mut Vec<GitChange>) {
    let choices = vec!["Stage all", "Stage selected"];
    let selection = dialoguer::Select::with_theme(
        &dialoguer::theme::ColorfulTheme::default(),
    )
    .items(&choices)
    .default(0)
    .interact()
    .unwrap();
    match selection {
        0 => Git::stage_all(),
        1 => show_stage_select_view(changes),
        _ => panic!("Invalid selection"),
    }
}

pub fn show_stage_select_view(changes: &mut Vec<GitChange>) {
    let mut checked = vec![false; changes.len()];
    for (i, change) in changes.iter().enumerate() {
        if change.status == GitChangeStatus::Staged {
            checked[i] = true;
        }
    }
    let selected =
        MultiSelect::with_theme(&dialoguer::theme::ColorfulTheme::default())
            .items(&changes)
            .defaults(&checked)
            .interact()
            .unwrap();
    for i in 0..changes.len() {
        if selected.contains(&i) {
            changes[i].stage();
        } else {
            changes[i].unstage();
        }
    }
}
