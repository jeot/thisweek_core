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

pub fn init_config() -> Result<Config, AppError> {
    // instantiate the configuration
    let path = default_config_path();
    let new_cfg = load_from_filepath(path).unwrap();
    println!("init config: {new_cfg:?}");
    // set or get the handle to arc
    if CONFIG.get().is_none() {
        CONFIG.set(ArcSwap::from_pointee(new_cfg.clone())).unwrap();
    } else {
        CONFIG.get().unwrap().store(Arc::new(new_cfg.clone()));
    }
    Ok(new_cfg)
}

pub fn get_config() -> Config {
    let gaurd = CONFIG.get_or_init(|| {
        println!("Init CONFIG (global OnceCell first run init)");
        let path = default_config_path();
        let new_cfg = load_from_filepath(path).unwrap();
        ArcSwap::from_pointee(new_cfg)
    });
    // println!("gaurd: {gaurd:?}");
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
            db_sqlite::create_db(&filepath)
        } else {
            // move current db
            // copy database file
            std::fs::copy(&current_db_path, &filepath)
                .map_err(|_| AppError::DatabaseFileCopyError)?;
            // change and save conifg
            config.database = filepath;
            set_config(config);
            save_config()?;
            // delete original database file
            std::fs::remove_file(&current_db_path)
                .map(|_| ())
                .map_err(|_| AppError::DatabaseFileRemoveError)
        }
    } else if exists && valid {
        // switch database
        // change and save conifg
        config.database = filepath;
        set_config(config);
        save_config()
    } else {
        Err(AppError::DatabaseFileInvalidError)
    }
}

pub fn save_config() -> Result<(), AppError> {
    let toml_str = toml::to_string(&get_config()).map_err(|_| AppError::ConfigTomlGenerateError)?;
    let path = default_config_path();
    fs::write(path, toml_str).map_err(|_| AppError::ConfigFileWriteError)
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

pub fn default_config_path() -> PathBuf {
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
    pub main_calendar_type: String,
    pub main_calendar_language: String,
    pub main_calendar_start_weekday: String,
    pub secondary_calendar_type: Option<String>,
    pub secondary_calendar_language: Option<String>,
    pub secondary_calendar_start_weekday: Option<String>, // todo: delete this, no need!
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
            secondary_calendar_start_weekday: self.secondary_calendar_start_weekday.clone(),
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            database: String::from("weeks_default_db"),
            main_calendar_type: "Gregorian".into(),
            main_calendar_language: "en".into(),
            main_calendar_start_weekday: "MON".into(),
            secondary_calendar_type: None,
            secondary_calendar_language: None,
            secondary_calendar_start_weekday: None,
        }
    }
}
