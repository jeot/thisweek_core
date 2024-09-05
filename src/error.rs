#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("this is a ConfigError from toml deserialization!")]
    BadConfigError(#[from] toml::de::Error),
    #[error("the config file not found!")]
    ConfigNotFoundError,
}
