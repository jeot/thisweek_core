use crate::prelude::Error as AppError;
use crate::prelude::Result as AppResult;
use serde::{Deserialize, Serialize};
use std::{fs, path::PathBuf};

use once_cell::sync::OnceCell;
static CONFIG: OnceCell<Config> = OnceCell::new();

pub fn get_config() -> Config {
    let config: &Config = CONFIG.get_or_init(|| {
        println!("Init CONFIG (global OnceCell first run init)");
        let path = default_config_path();
        let config = load_from_filepath(path).unwrap_or_default();
        println!("config: {config:?}");
        config
    });
    config.clone()
}

fn default_config_path() -> PathBuf {
    let path = homedir::get_my_home().unwrap().unwrap();
    path.join(".weeks.config")
}

// todo: no config file! create? ask user? probably first run!?
fn load_from_filepath(path: PathBuf) -> AppResult<Config> {
    println!("reading config file {}...", path.to_string_lossy());
    if let Ok(config) = fs::read_to_string(path) {
        toml::from_str(&config).map_err(AppError::ConfigSyntaxError)
    } else {
        Err(AppError::ConfigNotFoundError)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub database: String,
    pub main_calendar: String,
    pub secondary_calendar: Option<String>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            database: String::from("weeks_default_db"),
            main_calendar: "Gregorian".into(),
            secondary_calendar: None,
        }
    }
}
