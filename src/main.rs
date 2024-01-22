mod get_time;
mod git_operation;
mod path_fn;

use eyre::eyre;
use std::env::args;
use std::process::Command;

use get_time::fetch_current_time;
use git_operation::{git_add_commit, git_push};
use path_fn::get_path;

fn main() -> eyre::Result<()> {
    let mut args = args();
    args.next();

    let path_str = args.next().expect("ERROR: specify folder path on argument");
    let path = get_path(&path_str)?;
    let time_str = fetch_current_time();

    let mut cmd_add_commit = Command::new("git");

    let commit_output = git_add_commit(&mut cmd_add_commit, &time_str, &path)?;

    let err_commit_vec = commit_output.stderr;

    if !err_commit_vec.is_empty() {
        return Err(eyre!(
            "{err_commit}",
            err_commit = std::str::from_utf8(&err_commit_vec)?
        ));
    }

    let mut cmd_push = Command::new("git");
    let push_output = git_push(&mut cmd_push, &path)?;
    dbg!(&push_output);
    let push_err_vec = push_output.stderr;

    if !push_err_vec.is_empty() {
        return Err(eyre!(
            "{err_push}",
            err_push = std::str::from_utf8(&push_err_vec)?
        ));
    }

    let std_out_msg = std::str::from_utf8(&push_output.stdout)?;

    println!("{}", std_out_msg);

    Ok(())
}
