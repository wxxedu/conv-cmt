use crate::git::git::Git;

mod commit;
mod config;
mod git;
mod ui;

fn main() {
    println!("Has changes: {}", Git::has_changes());
}

