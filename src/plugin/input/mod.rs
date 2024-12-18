pub mod reader;

use super::{
    error::{ApplicationError, PluginError},
    Context, Payload,
};
use crate::config::Plugin;
use tokio::sync::mpsc;

/// InputPlugin receives messages and send them to the filters.
pub trait InputPlugin {
    /// Called when the processes is starting, useful for plugins that receives input
    /// from TCP port, for example.
    fn start(
        &mut self,
        context: Context,
        channel: mpsc::UnboundedSender<Payload>,
    ) -> Result<(), PluginError>;
    /// After the output, we need to `commit` the offset we already handled. So that if
    /// the process restarts, we know at which point should we retry operations.
    fn commit(&mut self, context: Context, id: String) -> Result<(), PluginError>;
    /// gracefully shutdown the plugin
    fn shutdown(&mut self, context: Context) -> Result<(), PluginError>;
    /// identifier is a unique id, identifying the plugin type and instance
    fn identifier(&self) -> String;
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
