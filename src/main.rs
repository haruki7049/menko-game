use bevy::prelude::*;
use clap::Parser;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _args: CLIArgs = CLIArgs::parse();

    App::new().add_plugins(MinimalPlugins).run();

    Ok(())
}

#[derive(Debug, Parser)]
#[clap(about, version, author)]
struct CLIArgs {}
