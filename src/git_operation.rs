use std::path::Path;
use std::process::{Command, Output};

fn change_curent_dir(git_command: &mut Command, temp_path: &Path) {
    git_command.current_dir(temp_path);
    let git_current_path = git_command.get_current_dir().unwrap();

    assert_eq!(temp_path, git_current_path);
}

pub fn git_add(git_command: &mut Command, temp_path: &Path) -> eyre::Result<Output> {
    change_curent_dir(git_command, temp_path);
    let output = git_command.arg("add").arg(".").output()?;
    Ok(output)
}

pub fn git_add_commit(
    git_command: &mut Command,
    time_now: &str,
    temp_path: &Path,
) -> eyre::Result<Output> {
    change_curent_dir(git_command, temp_path);
    let output = git_command.arg("commit").arg("-m").arg(time_now).output()?;
    Ok(output)
}

pub fn git_push(git_command: &mut Command, temp_path: &Path) -> eyre::Result<Output> {
    change_curent_dir(git_command, temp_path);
    let output = git_command.arg("push").output()?;
    Ok(output)
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::get_time::fetch_current_time;
    use std::fs::File;
    use std::io::Write;
    use std::path::Path;
    use tempdir::TempDir;

    fn git_init(git_command: &mut Command) {
        let output_init = git_command.arg("init").output().unwrap();
        dbg!(&output_init);
        check_valid_stdout(&output_init.stdout, "Initialized")
    }

    fn create_imaginary_file(temp_path: &Path) {
        let file_path = temp_path.join("my-temporary-note.txt");
        let mut tmp_file = File::create(file_path).unwrap();
        writeln!(tmp_file, "Brian was here. Briefly.").unwrap();
    }

    fn check_valid_stdout(slice: &[u8], checker: &str) {
        let get_str =
            std::str::from_utf8(slice).expect("ERROR: check_valid_stdout mus convert silce to str");

        println!("INFO: is contained {get_str}  > {checker} ");

        assert!(get_str.starts_with(checker));
    }

    #[test]
    fn git_test_git_commit() -> eyre::Result<()> {
        let temp_dir = TempDir::new("test_dir_commit")?;
        let temp_path = temp_dir.path();
        println!(
            "INFO: tempdir path = {temp_path}",
            temp_path = temp_path.display()
        );

        let mut git_cmd_init = Command::new("git");
        change_curent_dir(&mut git_cmd_init, &temp_path);
        git_init(&mut git_cmd_init);

        create_imaginary_file(&temp_path);

        let mut cmd_add = Command::new("git");
        let status = git_add(&mut cmd_add, &temp_path).and_then(|out| Ok(out.status))?;

        assert_eq!(true, status.success());

        let mut git_cmd_commit = Command::new("git");
        let time_str = fetch_current_time();
        let commit_output = git_add_commit(&mut git_cmd_commit, &time_str, &temp_path)
            .and_then(|op| Ok(op.stderr.len() == 0))?;

        assert_eq!(true, commit_output);

        Ok(())
    }
}
