mod cli;
mod command;
mod context;

use clap::Parser;

use cli::{Cli, Command};

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    match cli.command {
        Command::Identity { command } => {
            command::identity::execute(command).await;
        }

        Command::Credential { command } => {
            command::credential::execute(command).await;
        }

        Command::Client { command } => {
            command::client::execute(command).await;
        }
    }
}
