pub mod error;
pub mod input;

use std::{fs::read_to_string, path::PathBuf};
use tokio::runtime::Runtime;

use error::ApplicationError;
use input::InputPlugin;

use crate::config::{self, Plugin, PluginType};

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
            PluginType::Input => app.input = input::from_config(item.plugins)?,
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

pub struct Context {
    pub runtime: Runtime,
}
