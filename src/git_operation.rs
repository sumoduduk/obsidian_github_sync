use std::process::{Command, Output};

pub fn git_add_commit(git_command: &mut Command, time_now: &str) -> eyre::Result<Output> {
    let output = git_command
        .arg("commit")
        .arg("-a")
        .arg("-m")
        .arg(time_now)
        .output()?;
    Ok(output)
}

pub fn check_valid_stdout(slice: &[u8], checker: &str) {
    let get_str =
        std::str::from_utf8(slice).expect("ERROR: check_valid_stdout mus convert silce to str");

    println!("INFO: is contained {get_str}  > {checker} ");

    assert!(get_str.starts_with(checker));
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::get_time::fetch_current_time;
    use std::fs::File;
    use std::io::Write;
    use std::path::Path;
    use tempdir::TempDir;

    fn change_curent_dir(git_command: &mut Command, temp_path: &Path) {
        git_command.current_dir(temp_path);
        let git_current_path = git_command.get_current_dir().unwrap();

        assert_eq!(temp_path, git_current_path);
    }

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

        let mut git_cmd_commit = Command::new("git");
        change_curent_dir(&mut git_cmd_commit, &temp_path);
        let time_str = fetch_current_time();
        let commit_output = git_add_commit(&mut git_cmd_commit, &time_str)?;
        dbg!(&commit_output);

        let commit_output_len = commit_output.stderr.len();

        assert_eq!(0, commit_output_len);

        Ok(())
    }
}
