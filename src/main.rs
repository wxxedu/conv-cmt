use git::git_change::GitChanges;
use ui::git_stage::show_stage_all_or_select_view;

use crate::git::git::Git;

mod commit;
mod config;
mod git;
mod ui;

fn main() {
    let mut changes = Git::changes();
    if changes.is_empty() || !changes.has_changes() {
        println!("No changes to stage");
        return;
    }
    if changes.has_unstaged_changes() {
        show_stage_all_or_select_view(&mut changes);
    }
}
