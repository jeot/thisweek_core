use crate::calendar::Calendar;
use crate::calendar::CalendarLanguagePair;
use crate::db_sqlite;
use crate::language::Language;
use crate::prelude::Error as AppError;
use crate::prelude::Result as AppResult;
use arc_swap::ArcSwap;
use once_cell::sync::OnceCell;
use serde::{Deserialize, Serialize};
use std::path::Path;
use std::sync::Arc;
use std::{fs, path::PathBuf};

// global ref to static value
// static CONFIG: OnceCell<Config> = OnceCell::new();
pub static CONFIG: OnceCell<ArcSwap<Config>> = OnceCell::new();

pub fn set_config(new_cfg: Config) {
    if CONFIG.get().is_none() {
        CONFIG.set(ArcSwap::from_pointee(new_cfg.clone())).unwrap();
    } else {
        CONFIG.get().unwrap().store(Arc::new(new_cfg.clone()));
    }
}

pub fn reload_config_file() {
    println!("reloading config file...");
    let config_path = get_config_path();
    if let Ok(config) = load_from_filepath(config_path) {
        set_config(config);
        println!("success.");
    } else {
        println!("failed.");
    }
}

pub fn read_config_file_or_save_default_config_file() -> Result<Config, AppError> {
    let config_path = get_config_path();

    if let Ok(config) = load_from_filepath(config_path) {
        println!("config file available and ok");
        println!("config: {config:?}");
        Ok(config)
    } else {
        // no file or syntax err.
        // create new config file with defaults
        println!("no config file or syntax error!");
        let default_config = Config::default();
        println!("saving default config: {default_config:?}");
        save_config(default_config.clone())?;
        // return the default config
        println!("save successful.");
        Ok(default_config)
    }
}

pub fn check_database_file_is_valid_or_create_database_file(db_path: &str) -> Result<(), AppError> {
    if db_sqlite::is_correct_db(db_path) {
        Ok(())
    } else {
        db_sqlite::create_db(db_path)
    }
}

pub fn get_config() -> Config {
    let gaurd = CONFIG.get_or_init(|| {
        println!("Init CONFIG (global OnceCell first run init)");
        // note: here we crash! we can not read nor we can save!
        let new_cfg = read_config_file_or_save_default_config_file().unwrap();
        // note: here we crash! we need to make sure the database path directory exists
        if let Some(parent) = Path::new(&new_cfg.database).parent() {
            fs::create_dir_all(parent)
                .map_err(|_| AppError::DatabaseFileCreateError)
                .unwrap();
        }

        // note: for now, only checking if the directory exists, because db can not create and gives
        // error later!
        // check if the database is valid, if not create one, if error, crash.
        // check_database_file_is_valid_or_create_database_file(&new_cfg.database).unwrap();
        ArcSwap::from_pointee(new_cfg)
    });
    let gaurd = gaurd.load();
    gaurd.get_copy()
}

/// check if the new filepath exists or not
/// if exists, it should be a valid database and we only switch to it.
/// if the path don't exists, we will move our database to that location.
pub fn set_database_file(filepath: String) -> Result<(), AppError> {
    let mut config = get_config();
    let current_db_path = config.database;
    let current_db_valid = db_sqlite::is_correct_db(&current_db_path);
    let exists = Path::new(&filepath).exists();
    let valid = db_sqlite::is_correct_db(&filepath);
    if !exists {
        if !current_db_valid {
            // create new db
            println!("Attempting to creating new database file: {}", filepath);
            db_sqlite::create_db(&filepath)
        } else {
            // move current db
            // ensure target directory exists
            if let Some(parent) = Path::new(&filepath).parent() {
                fs::create_dir_all(parent).map_err(|_| AppError::DatabaseFileCopyError)?;
            }
            // copy database file
            std::fs::copy(&current_db_path, &filepath)
                .map_err(|_| AppError::DatabaseFileCopyError)?;
            // change and save config
            config.database = filepath;
            set_config(config.clone());
            save_config(config)?;
            // delete original database file
            std::fs::remove_file(&current_db_path)
                .map(|_| ())
                .map_err(|_| AppError::DatabaseFileRemoveError)
        }
    } else if exists && valid {
        // switch database
        // change and save config
        config.database = filepath;
        set_config(config.clone());
        save_config(config)
    } else {
        Err(AppError::DatabaseFileInvalidError)
    }
}

pub fn set_main_cal_config(
    main_calendar_type: String,
    main_calendar_language: String,
    main_calendar_start_weekday: String,
    weekdates_display_direction: String,
) -> Result<(), AppError> {
    let mut config = get_config();
    config.main_calendar_type = main_calendar_type;
    config.main_calendar_language = main_calendar_language;
    config.main_calendar_start_weekday = main_calendar_start_weekday;
    config.weekdates_display_direction = weekdates_display_direction;
    set_config(config.clone());
    save_config(config)
}

pub fn set_secondary_cal_config(
    secondary_calendar_type: Option<String>,
    secondary_calendar_language: Option<String>,
) -> Result<(), AppError> {
    let mut config = get_config();
    config.secondary_calendar_type = secondary_calendar_type;
    config.secondary_calendar_language = secondary_calendar_language;
    set_config(config.clone());
    save_config(config)
}

pub fn set_items_display_direction_config(items_direction: String) -> Result<(), AppError> {
    let mut config = get_config();
    config.items_display_direction = items_direction;
    set_config(config.clone());
    save_config(config)
}

pub fn save_config(config: Config) -> Result<(), AppError> {
    let toml_str = toml::to_string(&config).map_err(|e| {
        println!("Failed to serialize config to TOML: {}", e);
        AppError::ConfigTomlGenerateError
    })?;

    let config_path = get_config_path();
    println!(
        "Attempting to save config to: {}",
        config_path.to_string_lossy()
    );

    // Ensure the parent directory exists
    if let Some(parent) = config_path.parent() {
        println!("Creating directory if needed: {}", parent.to_string_lossy());
        fs::create_dir_all(parent).map_err(|e| {
            println!("Failed to create directory: {}", e);
            AppError::ConfigFileSaveError
        })?;
    }

    // Write the config file
    fs::write(&config_path, toml_str).map_err(|e| {
        println!("Failed to write config file: {}", e);
        println!("Path: {}", config_path.to_string_lossy());
        AppError::ConfigFileSaveError
    })
}

pub fn get_main_cal_lang_pair() -> CalendarLanguagePair {
    let calendar: Calendar = get_config().main_calendar_type.into();
    let language: Language = get_config().main_calendar_language.into();
    CalendarLanguagePair { calendar, language }
}

pub fn get_second_cal_lang_pair() -> Option<CalendarLanguagePair> {
    get_config().secondary_calendar_type.map(|cal| {
        let language: Language = get_config()
            .secondary_calendar_language
            .unwrap_or_default()
            .into();
        let calendar: Calendar = cal.into();
        CalendarLanguagePair { calendar, language }
    })
}

fn default_config_data_path() -> AppResult<(PathBuf, PathBuf)> {
    // Retrieve project-specific directories
    if let Some(proj_dirs) = directories::ProjectDirs::from("", "", "ThisWeek") {
        // Config directory: ~/.config/ThisWeek on Linux, ~/Library/Application Support/ThisWeek on macOS, and %AppData%\ThisWeek on Windows
        let config_path = proj_dirs.config_dir().join("config.toml");
        // println!("Config directory: {}", config_dir.display());

        // Data directory: similar to config_dir but used for storing database and other data files
        let data_path = proj_dirs.data_dir().join("thisweek.db");
        // println!("Data directory: {}", data_dir.display());
        Ok((config_path, data_path))
    } else {
        eprintln!("Could not determine project directories!");
        Err(AppError::DefaultAppPathError)
    }
}

fn load_from_filepath(path: PathBuf) -> AppResult<Config> {
    println!("reading config file {}...", path.to_string_lossy());
    if let Ok(config) = fs::read_to_string(path) {
        let config = toml::from_str(&config).map_err(|e| AppError::ConfigSyntaxError(e))?;
        Ok(config)
    } else {
        Err(AppError::ConfigNotFoundError)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub database: String,
    pub main_calendar_type: String,
    pub main_calendar_language: String,
    pub main_calendar_start_weekday: String,
    pub secondary_calendar_type: Option<String>,
    pub secondary_calendar_language: Option<String>,
    pub weekdates_display_direction: String,
    pub items_display_direction: String,
}

impl Config {
    pub fn get_copy(&self) -> Config {
        Config {
            database: self.database.clone(),
            main_calendar_type: self.main_calendar_type.clone(),
            main_calendar_language: self.main_calendar_language.clone(),
            main_calendar_start_weekday: self.main_calendar_start_weekday.clone(),
            secondary_calendar_type: self.secondary_calendar_type.clone(),
            secondary_calendar_language: self.secondary_calendar_language.clone(),
            weekdates_display_direction: self.weekdates_display_direction.clone(),
            items_display_direction: self.items_display_direction.clone(),
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        let data_path = default_config_data_path()
            .unwrap()
            .1
            .to_string_lossy()
            .into_owned();
        Self {
            database: data_path,
            main_calendar_type: "Gregorian".into(),
            main_calendar_language: "en".into(),
            main_calendar_start_weekday: "MON".into(),
            secondary_calendar_type: None,
            secondary_calendar_language: None,
            weekdates_display_direction: "ltr".into(),
            items_display_direction: "auto".into(),
        }
    }
}

pub fn get_config_path() -> PathBuf {
    default_config_data_path().unwrap().0
}
