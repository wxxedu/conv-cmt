use commit::{
    builder::CommitBuilder, cmt_type::CommitType, commit::Commit,
    strategy::CaseStrategy,
};
use git::git_change::GitChanges;
use ui::{
    git_commit::{ask_commit_type, ask_description, ask_scope, ask_subject},
    git_stage::show_stage_all_or_select_view,
};

use crate::{git::git::Git, ui::git_push::ask_push};

mod commit;
mod config;
mod git;
mod ui;

fn logic() {
    // add
    let mut changes = Git::changes();
    if changes.is_empty() || !changes.has_changes() {
        println!("No changes to stage");
        return;
    }
    if changes.has_unstaged_changes() {
        show_stage_all_or_select_view(&mut changes);
    }

    // commit
    let strat = CaseStrategy::Lowercase;
    let mut commit_builder = &mut Commit::builder(strat);
    let types = vec![
        CommitType::new("feat", Some("A new feature")),
        CommitType::new("fix", Some("A bug fix")),
        CommitType::new("docs", Some("Documentation only changes")),
        CommitType::new("style", Some("Changes that do not affect the meaning of the code (white-space, formatting, missing semi-colons, etc)")),
        CommitType::new("refactor", Some("A code change that neither fixes a bug nor adds a feature")),
        CommitType::new("perf", Some("A code change that improves performance")),
        CommitType::new("test", Some("Adding missing tests or correcting existing tests")),
        CommitType::new("build", Some("Changes that affect the build system or external dependencies (example scopes: gulp, broccoli, npm)")),
        CommitType::new("ci", Some("Changes to our CI configuration files and scripts (example scopes: Travis, Circle, BrowserStack, SauceLabs)")),
        CommitType::new("chore", Some("Other changes that don't modify src or test files")),
        CommitType::new("revert", Some("Reverts a previous commit")),
    ];
    let res = ask_commit_type(&types);
    commit_builder = commit_builder.commit_type(res);
    let scope = ask_scope();
    let mut scope_str = "".to_string();
    if let Some(scope) = scope {
        scope_str = strat.apply(&scope);
        commit_builder = commit_builder.scope(&scope_str).unwrap();
    }
    let subject = ask_subject();
    let subject = strat.apply(&subject);
    commit_builder = commit_builder.subject(&subject).unwrap();
    let description = ask_description();
    commit_builder = commit_builder.description(&description);
    let commit = commit_builder.build().unwrap();
    Git::commit(&commit);
    let push = ask_push();
    if push {
        Git::push();
    }
}

fn main() {
    logic();
}
