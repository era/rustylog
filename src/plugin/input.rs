use crate::config::{self, AttributeValue, Plugin};

use super::error::ApplicationError;

pub trait InputPlugin {
    /// Called when the processes is starting, useful for plugins that receives input
    /// from TCP port, for example.
    fn init(&mut self, config: Vec<(String, config::AttributeValue)>) -> Result<(), ()>;
    /// After the output, we need to `commit` the offset we already handled. So that if
    /// the process restarts, we know at which point should we retry operations.
    fn commit(&mut self) -> Result<(), ()>;
    /// Return a `Producer` so filter can consume the inputs.
    fn producer(&mut self) -> Result<(), ()>; // TODO probably use a channel here
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
