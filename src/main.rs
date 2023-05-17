use std::{env, ffi::OsString};

use commit::{cmt_type::CommitType, strategy::CaseStrategy};
use console::Term;
use dialoguer::theme::ColorfulTheme;
use ui::git_ui::GitUI;
mod commit;
mod git;
mod ui;

fn main() {
    // prep
    let mut term = Term::stdout();
    let theme = ColorfulTheme::default();
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
    let strat = CaseStrategy::Lowercase;
    let editor =
        env::var_os("EDITOR").unwrap_or_else(|| OsString::from("nvim"));

    let mut ui = GitUI::new(&mut term, theme, &types, strat, editor);

    ui.show();
}
