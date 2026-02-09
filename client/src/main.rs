mod checker;
mod daemon_client;
mod args_handler;
mod terminal_ui;

use clap::Parser;
use args_handler::CLArgs;
use tokio;

#[tokio::main]
async fn main() {
    let args = CLArgs::parse();
    match args_handler::handle(args).await {
        Ok(res) => println!("{res}"),
        Err(err) => println!("Error: {err}"),
    }
}
