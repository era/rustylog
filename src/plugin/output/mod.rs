use std::{
    collections::HashMap,
    io::{self, Stdout, Write},
};

use crate::config::{AttributeValue, Plugin};

use super::{
    error::{ApplicationError, ProcessError},
    Payload,
};

pub fn from_config(plugins: Vec<Plugin>) -> Result<Vec<Box<dyn OutputPlugin>>, ApplicationError> {
    let mut output_plugins: Vec<Box<dyn OutputPlugin>> = vec![];
    for plugin in plugins {
        let input = match plugin.name.as_str() {
            "stdin" => StdoutPlugin::new(plugin.attributes),
            name @ _ => return Err(ApplicationError::PluginNotFound(name.to_string())),
        };

        output_plugins.push(Box::new(input));
    }

    Ok(output_plugins)
}

pub trait OutputPlugin {
    fn consume(&mut self, payload: &Payload) -> Result<(), ProcessError>;
}

pub struct StdoutPlugin {
    stdout: Stdout,
}

impl StdoutPlugin {
    fn new(_config: HashMap<String, AttributeValue>) -> Self {
        let stdout = io::stdout();
        Self { stdout }
    }
}

impl OutputPlugin for StdoutPlugin {
    fn consume(&mut self, payload: &Payload) -> Result<(), ProcessError> {
        self.stdout.write_all(payload.data.as_bytes())?;

        Ok(())
    }
}
