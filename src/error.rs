use thiserror::Error;

#[derive(Debug, Error)]
pub enum KvError {
    #[error("I/O error")]
    IoError(#[from] std::io::Error),
    #[error("Parse config error")]
    ConfigError(#[from] toml::de::Error),
}
