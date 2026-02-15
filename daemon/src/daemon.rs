use tokio::{fs::remove_file, io::{AsyncReadExt, AsyncWriteExt}, net::UnixListener};
use lib::{command::Command, token::Token};
use crate::prelude::*;
use api_request::{get_token, get_contest};


async fn execute_command(command: &Command, token: &Token) -> Result<Box<str>> {
    match command {
        Command::Submit { contest, task, code } => {
            todo!();
        },
        Command::GetInstance { contest } => {
            Ok(serde_json::to_string(&get_contest(token, contest).await?)?.into())
        }
    }
}

pub async fn start(token: Option<Box<str>>, name: Option<Box<str>>, password: Option<Box<str>>) -> Result<Box<str>> {
    let _ = remove_file(UNIX_SOCKET_PATH).await;
    let listener = UnixListener::bind(UNIX_SOCKET_PATH).context("while binding Unix Socket")?;

    let token = match token {
        Some(token) => Token(token),
        None => {
            let (Some(name), Some(password)) = (name, password) else {
                return Err(anyhow!("Can't start waff_daemon without specified token or name and password"));
            };
            
            get_token(&name, &password).await?
        }
    };


    log::info!("waff_daemon started.");
    while let Ok((mut stream, _)) = listener.accept().await {
        let mut command_string = String::new();
        let len = stream.read_to_string(&mut command_string).await.context("while trying to read UnixStream")?;
        log::trace!("Readed message {command_string} with len {len}");
        let command: Command = serde_json::from_str(&command_string).context("while parsing command")?;
        match execute_command(&command, &token).await {
            Ok(res) => {
                match stream.write_all(res.as_bytes()).await {
                    Err(err) => log::error!("Err: {err}"),
                    Ok(..) => log::info!("{res}"),
                };
            },
            Err(err) => {
                log::error!("Err: {err}");
            }
        }
    }
    Ok("waff_daemon stopped.".into())
}
