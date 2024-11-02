use std::{fmt::Display, fs::read_to_string, path::PathBuf};

use thiserror::Error;

use crate::config::{
    self, language::ConfigParseError, AttributeValue, Plugin, PluginSection, PluginType,
};

#[derive(Debug, Error)]
#[non_exhaustive]
pub enum ApplicationError {
    #[error("Configuration Error: {0}")]
    ConfigError(#[from] ConfigParseError),

    #[error("IO Error: {0}")]
    IoError(#[from] std::io::Error),
}

trait InputPlugin {
    fn init(&mut self, config: Vec<(String, config::AttributeValue)>) -> Result<(), ()>;
    fn commit(&mut self) -> Result<(), ()>;
    fn produce(&mut self) -> Result<(), ()>; // TODO probably use a channel here
}
trait FilterPlugin {}
trait OutputPlugin {}

#[derive(Default)]
pub struct Application {
    input: Vec<Box<dyn InputPlugin>>,
    filters: Vec<Box<dyn FilterPlugin>>,
    output: Vec<Box<dyn OutputPlugin>>,
}

pub fn from_config(config: PathBuf) -> Result<Application, ApplicationError> {
    let mut app = Application::default();

    let config = read_to_string(config)?;
    let config = config::language::parse_logstash_config(&config)?;
    for item in config {
        match item.plugin_type {
            PluginType::Input => app.input = input_plugins(item.plugins)?,
            PluginType::Output => app.output = output_plugins(item.plugins)?,
            PluginType::Filter => app.filters = filter_plugins(item.plugins)?,
        };
    }
    Ok(app)
}

fn filter_plugins(plugins: Vec<Plugin>) -> Result<Vec<Box<dyn FilterPlugin>>, ApplicationError> {
    todo!()
}

fn output_plugins(plugins: Vec<Plugin>) -> Result<Vec<Box<dyn OutputPlugin>>, ApplicationError> {
    todo!()
}

fn input_plugins(plugins: Vec<Plugin>) -> Result<Vec<Box<dyn InputPlugin>>, ApplicationError> {
    todo!()
}

fn find_input_plugin(
    name: String,
    attributes: Vec<(String, AttributeValue)>,
) -> Result<Box<dyn InputPlugin>, ApplicationError> {
    todo!()
}
