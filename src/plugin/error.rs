use thiserror::Error;

use crate::config::language::ConfigParseError;

#[derive(Debug, Error)]
#[non_exhaustive]
pub enum ApplicationError {
    #[error("Configuration Error: {0}")]
    ConfigError(#[from] ConfigParseError),

    #[error("IO Error: {0}")]
    IoError(#[from] std::io::Error),
}

#[derive(Debug, Error)]
#[non_exhaustive]
pub enum PluginError {
    #[error("Plugin was not correctly initialized: {0}")]
    NotInitialized(String),
}
