use serde::Deserialize;
use serde_json::Value;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

#[derive(Debug, Deserialize)]
pub struct GithubConfig {
    owner: String,
    repo: String,
    key: String,
}

pub fn load_config(path: &Path) -> eyre::Result<KeyStruct> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let json_res = serde_json::from_reader(reader)?;
    Ok(json_res)
}

#[cfg(test)]
mod test {
    use super::*;
    use dotenvy::dotenv;
    use eyre::OptionExt;
    use std::env;

    #[test]
    fn test_config_load() -> eyre::Result<()> {
        dotenv().ok();

        let path_to_json =
            env::var("PATH_JSON").expect("must have path to json environment variable");

        let path = Path::new(&path_to_json);

        let json_str = load_config(&path)?;

        assert_eq!("secret", &json_str.key);

        Ok(())
    }
}
