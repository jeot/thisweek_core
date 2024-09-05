use crate::prelude::Error as AppError;
use crate::prelude::Result as AppResult;
use serde::{Deserialize, Serialize};
use std::{fs, path::PathBuf};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Config {
    database: String,
    main_calendar: Option<String>,
    secondary_calendar: Option<String>,
}

pub fn load(path: PathBuf) -> AppResult<Config> {
    if let Ok(config) = fs::read_to_string(path) {
        toml::from_str(&config).map_err(AppError::BadConfigError)
    } else {
        // no config file, return error
        Err(AppError::ConfigNotFoundError)
        // todo: no config file, create
        /*
        let config = Config {
            database: Some("".to_string()),
            main_calendar: Some("Gregorian".to_string()),
            secondary_calendar: None,
        };
        let toml = toml::to_string(&config).unwrap();
        fs::write(path, toml);
        Ok(config)
        */
    }
}
