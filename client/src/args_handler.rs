use std::path::Path;

use lib::instance::Instance;
use lib::ts_api::ContestWithTasks;
use tokio::fs::File;
use tokio::io::AsyncReadExt;

use crate::checker;
use anyhow::{Context, Result};
use crate::terminal_ui;
use crate::daemon_client;
use lib::command::Command as ApiCommand
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
            Instance::from_api(serde_json::from_str(
                daemon_client::send_command(ApiCommand::GetInstance {
                    contest,
                }).await?
            )).save_to(INSTANCE_FOLDER);
            Ok(format!("Succesfuly got and saved instance").into())
        },
        Command::Submit { task, path } => {
            let mut file = File::open(path.clone()).await.context("can't open submitted file")?;
            let mut code = String::new();
            let contest_id = instance::get_contest_id(INSTANCE_FOLDER).await?;
            file.read_to_string(&mut code).await.context("while reading solution file")?;
            daemon_client::send_command(ApiCommand::Submit {
                code: code.into(), task, contest: contest_id,
            }).await?;
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
