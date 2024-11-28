#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("can not get OS default application path!")]
    DefaultAppPathError,

    #[error("this is a ConfigError from toml deserialization!")]
    ConfigSyntaxError(#[from] toml::de::Error),
    #[error("the config file not found!")]
    ConfigNotFoundError,
    #[error("can not generate toml file")]
    ConfigTomlGenerateError,
    #[error("can not save (write) config toml file")]
    ConfigFileSaveError,

    #[error("provided days range is not correct")]
    BadDaysRangeError,
    #[error("provided days range is very long: {} days", self)]
    LongDaysRangeError(i32),

    #[error("invalid timestamp: sec: {sec}, nano: {nano}")]
    InvalidTimestampError { sec: i64, nano: u32 },

    #[error("database error: {0}")]
    DatabaseInsertError(String),
    #[error("database error: {0}")]
    DatabaseSelectError(String),
    #[error("can not copy database file")]
    DatabaseFileCopyError,
    #[error("can not create database file")]
    DatabaseFileCreateError,
    #[error("can not delete database file")]
    DatabaseFileRemoveError,
    #[error("the data file is not valid")]
    DatabaseFileInvalidError,
}
