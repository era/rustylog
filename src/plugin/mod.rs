pub mod error;
pub mod input;
pub mod output;

use futures::stream::{select_all, StreamExt};
use output::OutputPlugin;
use std::{collections::HashMap, fs::read_to_string, path::PathBuf};
use tokio::{runtime::Handle, sync::mpsc};

use error::ApplicationError;
use input::InputPlugin;

use crate::config::{self, Plugin, PluginType};

trait FilterPlugin {}

#[derive(Default)]
pub struct Application {
    input: Vec<Box<dyn InputPlugin>>,
    filters: Vec<Box<dyn FilterPlugin>>,
    output: Vec<Box<dyn OutputPlugin>>,
}

impl Application {
    pub async fn start(mut self, handle: Handle) -> Result<(), ApplicationError> {
        let ctx = Context { runtime: handle };

        let (sender, mut receiver) = mpsc::unbounded_channel();

        Application::run_input(ctx.clone(), &mut self.input, sender)?;

        let mut input_plugins: HashMap<String, Box<dyn InputPlugin>> = self
            .input
            .into_iter()
            .map(|i| (i.identifier(), i))
            .collect();

        while let Some(payload) = receiver.recv().await {
            let input = input_plugins
                .get_mut(&payload.plugin_id)
                .expect("plugin must exist in hashmap");
            for out in self.output.iter_mut() {
                out.as_mut().consume(&payload)?;
            }
            input.as_mut().commit(ctx.clone(), payload.id)?;
        }

        Ok(())
    }

    fn run_input(
        ctx: Context,
        plugins: &mut Vec<Box<dyn InputPlugin>>,
        sender: mpsc::UnboundedSender<Payload>,
    ) -> Result<(), ApplicationError> {
        for plugin in plugins {
            plugin.start(ctx.clone(), sender.clone())?;
        }

        Ok(())
    }
}

pub fn from_config(config: PathBuf) -> Result<Application, ApplicationError> {
    let mut app = Application::default();

    let config = read_to_string(config)?;
    let config = config::language::parse_logstash_config(&config)?;
    for item in config {
        match item.plugin_type {
            PluginType::Input => app.input = input::from_config(item.plugins)?,
            PluginType::Output => app.output = output::from_config(item.plugins)?,
            PluginType::Filter => app.filters = filter_plugins(item.plugins)?,
        };
    }
    Ok(app)
}

fn filter_plugins(plugins: Vec<Plugin>) -> Result<Vec<Box<dyn FilterPlugin>>, ApplicationError> {
    todo!()
}

#[derive(Clone)]
pub struct Context {
    pub runtime: Handle,
}

impl Context {
    fn new() -> Self {
        let runtime = Handle::current();
        Self { runtime }
    }
}

#[derive(Clone, Debug)]
pub struct Payload {
    pub id: String,
    pub data: String,
    pub plugin_id: String,
}
