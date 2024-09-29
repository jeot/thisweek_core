#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("this is a ConfigError from toml deserialization!")]
    ConfigSyntaxError(#[from] toml::de::Error),
    #[error("the config file not found!")]
    ConfigNotFoundError,
    #[error("can not generate toml file")]
    ConfigTomlGenerateError,
    #[error("can not write config toml file")]
    ConfigFileWriteError,

    #[error("provided days range is not correct")]
    BadDaysRangeError,
    #[error("provided days range is very long: {} days", self)]
    LongDaysRangeError(i32),

    #[error("invalid timestamp: sec: {sec}, nano: {nano}")]
    InvalidTimestampError { sec: i64, nano: u32 },

    #[error("database error: {0}")]
    DatabaseError(String),
    #[error("can not copy database file")]
    DatabaseFileCopyError,
    #[error("can not delete database file")]
    DatabaseFileRemoveError,
}
