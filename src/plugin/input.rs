use super::{
    error::{ApplicationError, PluginError},
    Context,
};
use crate::config::{self, AttributeValue, Plugin};
use std::io::{self, BufRead};

pub trait InputPlugin {
    /// Called when the processes is starting, useful for plugins that receives input
    /// from TCP port, for example.
    fn init(
        &mut self,
        context: Context,
        config: Vec<(String, config::AttributeValue)>,
    ) -> Result<Self, PluginError>
    where
        Self: Sized;
    /// After the output, we need to `commit` the offset we already handled. So that if
    /// the process restarts, we know at which point should we retry operations.
    fn commit(&mut self, context: Context) -> Result<(), PluginError>;
    /// Return a `Producer` so filter can consume the inputs.
    fn producer(&mut self, context: Context) -> Result<(), PluginError>; // TODO probably use a channel here
    /// gracefully shutdown the plugin
    fn shutdown(&mut self, context: Context) -> Result<(), PluginError>;
}

pub fn from_config(plugins: Vec<Plugin>) -> Result<Vec<Box<dyn InputPlugin>>, ApplicationError> {
    todo!()
}

fn find_input_plugin(
    name: String,
    attributes: Vec<(String, AttributeValue)>,
) -> Result<Box<dyn InputPlugin>, ApplicationError> {
    todo!()
}

#[derive(Clone)]
pub struct StdinPlugin {
    //TODO channel to stop the operation
}

impl InputPlugin for StdinPlugin {
    fn init(
        &mut self,
        context: Context,
        _config: Vec<(String, config::AttributeValue)>,
    ) -> Result<Self, PluginError> {
        let plugin = StdinPlugin {};
        let plugin_runtime = plugin.clone();

        context.runtime.spawn(async move {
            let stdin = io::stdin();
            // todo wait for a cancel operation or new lines
            for lines in stdin.lock().lines() {
                //....
            }
        });

        Ok(plugin.clone())
    }

    fn commit(&mut self, context: Context) -> Result<(), PluginError> {
        todo!()
    }

    fn producer(&mut self, context: Context) -> Result<(), PluginError> {
        todo!()
    }

    fn shutdown(&mut self, context: Context) -> Result<(), PluginError> {
        todo!()
    }
}
