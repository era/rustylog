mod config;
mod plugin;

use std::path::PathBuf;
use tokio::runtime::Handle;

use clap::Parser;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Sets a custom config file
    #[arg(short, long, value_name = "FILE")]
    config: PathBuf,
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    let mut app = plugin::from_config(cli.config).unwrap();
    app.start(Handle::current()).await.unwrap();

    app.process().unwrap();
}
