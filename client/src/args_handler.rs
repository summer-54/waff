use std::path::Path;

use tokio::fs::File;

use crate::checker;
use anyhow::{Context, Result};
use crate::terminal_ui;
use crate::daemon_client;
use lib::{
    instance,
    formatter,
    defaults::INSTANCE_FOLDER,
};

#[derive(clap::Parser)]
pub struct CLArgs {
    #[command(subcommand)]
    command: Command,
}
#[derive(clap::Subcommand)]
pub enum Command {
    New {
        #[arg(short, long)]
        contest: Box<str>,
    },
    Submit {
        //#[arg(short, long)]
        task: Box<str>,
        //#[arg(short, long)]
        path: Box<Path>,
    },
    Check {
        #[arg(short, long)]
        task: Box<str>,
        #[arg(short, long)]
        path: Box<Path>,
    },
    Tui,
}


pub async fn handle(args: CLArgs) -> Result<Box<str>> {
    match args.command {
        Command::New { contest } => {
            daemon_client::download_instance(&contest).await?.save_to(&INSTANCE_FOLDER.to_string()).await?;
            Ok(format!("Succesfuly got and saved instance").into())
        },
        Command::Submit { task, path } => {
            let mut file = File::open(path.clone()).await.context("can't open submitted file")?;
            daemon_client::submit(&instance::get_contest_id(INSTANCE_FOLDER.into()).await?, &task, &mut file).await.context("while submitting solution")
        },
        Command::Tui => {
            terminal_ui::start().await
        },
        Command::Check { task, path } => {
            Ok(formatter::format_tests_verdicts(checker::check_on_samples(&task, &path).await?))
        },
        _ => {
            Ok("Unhandled command".into())
        },
    }
}
