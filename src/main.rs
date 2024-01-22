mod get_config;
mod get_time;
mod git_operation;
mod path_fn;

use std::env::args;
use std::process::Command;

use get_time::fetch_current_time;
// use git_opertaion::{git_add_commit, git_push};
use path_fn::get_path;

fn main() -> eyre::Result<()> {
    todo!();
    // let mut args = args();
    // args.next();
    //
    // let path_str = args.next().expect("ERROR: specify folder path on argument");
    // let path = get_path(&path)?;
    // let time_str = fetch_current_time()?;
    //
    // let mut cmd_add_commit = Command::new("git");
    //
    // let output_add_commit = git_add_commit(cmd_add_commit, &time_str, &path);
}
