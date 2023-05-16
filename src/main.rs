use crate::git::git::Git;

mod commit;
mod config;
mod git;
mod ui;

fn main() {
    let changes = Git::changes();
    for change in changes {
        println!("{}", change);
    }
    Git::stage_all();
}
