use eyre::OptionExt;
use std::path::PathBuf;

pub fn get_path(path_str: &str) -> eyre::Result<PathBuf> {
    let path = PathBuf::from(path_str);
    let _ = path
        .is_dir()
        .then_some(|p: PathBuf| p)
        .ok_or_eyre("folder not exists");
    Ok(path)
}

#[cfg(test)]
mod test {
    use super::*;
    use dotenvy::dotenv;
    use std::env::var;

    #[test]
    fn test_path_is_exist() -> eyre::Result<()> {
        dotenv().ok();
        let path_str = var("PATH_FOLDER").expect("ERROR: PATH_FOLDER environment must present");

        let path = get_path(&path_str)?;

        let goal = "/home";
        let parent = path.parent().unwrap();
        let display = format!("{}", parent.display());

        assert_eq!(goal, &display);

        Ok(())
    }
}
