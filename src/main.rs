mod git_operation;
mod path_fn;

use eyre::eyre;
use std::env::args;

use git_operation::Git;
use path_fn::get_path;

fn main() -> eyre::Result<()> {
    let mut args = args();
    args.next();

    let path_str = args.next().expect("ERROR: specify folder path on argument");
    let path = get_path(&path_str)?;

    let add_output = Git::Add.proceed(&path)?;

    if !add_output.status.success() {
        return Err(eyre!(
            "{error_add}",
            error_add = std::str::from_utf8(&add_output.stderr)?
        ));
    };

    let commit_output = Git::Commit.proceed(&path)?;

    if !commit_output.stderr.is_empty() {
        return Err(eyre!(
            "{err_commit}",
            err_commit = std::str::from_utf8(&commit_output.stderr)?
        ));
    }

    let push_output = Git::Push.proceed(&path)?;

    let std_out_msg = std::str::from_utf8(&push_output.stderr)?;

    println!("{}", std_out_msg);

    Ok(())
}
