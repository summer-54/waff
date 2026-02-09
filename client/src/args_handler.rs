use std::path::Path;

use tokio::fs::File;


use crate::checker;
use anyhow::{Context, anyhow};
use crate::terminal_ui;
use crate::daemon_client;
use crate::defaults::INSTANCE_FOLDER;

#[derive(clap::Parser)]
pub struct CLArgs {
    #[command(subcommand)]
    command: Command,
}
#[derive(clap::Subcommand)]
pub enum Command {
    New {
        #[arg(short, long)]
        contest: String,
    },
    Submit {
        //#[arg(short, long)]
        task: String,
        //#[arg(short, long)]
        path: Box<Path>,
    },
    Check {
        #[arg(short, long)]
        task: String,
        #[arg(short, long)]
        path: Box<Path>,
    },
    Tui,
}


pub async fn handle(args: CLArgs) -> Result<String> {
    match args.command {
        Command::New { contest } => {
            daemon_client::download_instance(contest).await?.save_to(&INSTANCE_FOLDER.to_string()).await?;
            Ok(format!("Succesfuly got and saved instance"))
        },
        Command::Submit { task, path } => {
            let mut file = File::open(path.clone()).await.context("can't open submitted file")?
            daemon_client::submit(instance::get_contest_id(INSTANCE_FOLDER.to_string()).await?, task, &mut file).await
        },
        Command::Tui => {
            terminal_ui::start().await
        },
        Command::Check { task, path } => {
            Ok(format!("{}", formatter::format_tests_verdicts(checker::check_on_samples(task, &path).await?)))
        },
        _ => {
            Ok("Unhandled command".to_string())
        },
    }
}
