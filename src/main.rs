mod config;
mod plugin;

use std::path::PathBuf;

use clap::Parser;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Sets a custom config file
    #[arg(short, long, value_name = "FILE")]
    config: PathBuf,
}

fn main() {
    let cli = Cli::parse();
    plugin::from_config(cli.config).unwrap();
    println!("Hello, world!");
}
