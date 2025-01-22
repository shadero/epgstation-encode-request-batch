use std::path::PathBuf;

use anyhow::Result;

use super::model::{Config, EncodeRule};

pub fn create_config_file(path: &PathBuf) -> Result<()> {
    let config = Config {
        epgstation_url: "http://localhost:8888".to_string(),
        default_encode_mode: "H.264".to_string(),
        encode_rule: vec![
            EncodeRule {
                encode_mode: "H.265".to_string(),
                rules: Some(vec![1, 2, 3]),
                no_rule: None,
            },
            EncodeRule {
                encode_mode: "AV1".to_string(),
                rules: None,
                no_rule: Some(true),
            },
        ],
    };

    let toml = toml::to_string(&config)?;
    std::fs::write(path, toml)?;

    Ok(())
}

pub fn load_config(path: &PathBuf) -> Result<Config> {
    let config = std::fs::read_to_string(path)?;
    let config = toml::from_str(&config)?;

    Ok(config)
}
