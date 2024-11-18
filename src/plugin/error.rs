use thiserror::Error;

use crate::config::language::ConfigParseError;

#[derive(Debug, Error)]
#[non_exhaustive]
pub enum ApplicationError {
    #[error("Configuration Error: {0}")]
    ConfigError(#[from] ConfigParseError),

    #[error("Plugin not found: {0}")]
    PluginNotFound(String),

    #[error("Error on Plugin: {0}")]
    PluginError(#[from] PluginError),

    #[error("IO Error: {0}")]
    IoError(#[from] std::io::Error),
}

#[derive(Debug, Error)]
#[non_exhaustive]
pub enum PluginError {
    #[error("Plugin was not correctly initialized: {0}")]
    NotInitialized(String),
}

#[derive(Debug, Error)]
#[non_exhaustive]
pub enum ProcessError {
    #[error("Error while performing IO: {0}")]
    IoError(#[from] std::io::Error),
}
