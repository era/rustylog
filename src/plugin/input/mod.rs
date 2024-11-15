pub mod reader;

use std::collections::HashMap;

use super::{
    error::{ApplicationError, PluginError},
    Context,
};
use crate::config::Plugin;
use tokio::sync::broadcast;

/// InputPlugin receives messages and send them to the filters.
pub trait InputPlugin {
    /// Called when the processes is starting, useful for plugins that receives input
    /// from TCP port, for example.
    fn start(&mut self, context: Context) -> Result<broadcast::Receiver<String>, PluginError>;
    /// After the output, we need to `commit` the offset we already handled. So that if
    /// the process restarts, we know at which point should we retry operations.
    fn commit(&mut self, context: Context) -> Result<(), PluginError>;
    /// Return a `Producer` so filter can consume the inputs.
    fn subscribe(&mut self, context: Context) -> Result<broadcast::Receiver<String>, PluginError>;
    /// gracefully shutdown the plugin
    fn shutdown(&mut self, context: Context) -> Result<(), PluginError>;
}

pub fn from_config(plugins: Vec<Plugin>) -> Result<Vec<Box<dyn InputPlugin>>, ApplicationError> {
    let mut input_plugins: Vec<Box<dyn InputPlugin>> = vec![];
    for plugin in plugins {
        let input = match plugin.name.as_str() {
            "stdin" => reader::StdinPlugin::new(plugin.attributes),
            name @ _ => return Err(ApplicationError::PluginNotFound(name.to_string())),
        };

        input_plugins.push(Box::new(input));
    }

    Ok(input_plugins)
}
