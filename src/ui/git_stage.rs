use dialoguer::MultiSelect;

use crate::git::git_change::{GitChange, GitChangeStatus};

pub fn show_stage_select_view(changes: &mut Vec<GitChange>) {
    let mut checked = vec![false; changes.len()];
    for (i, change) in changes.iter().enumerate() {
        if change.status == GitChangeStatus::Staged {
            checked[i] = true;
        }
    }
    let selected =
        MultiSelect::with_theme(&dialoguer::theme::ColorfulTheme::default())
            .with_prompt("Select changes to stage (press space to select/deselect, a to toggle all, and enter to continue)")
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
