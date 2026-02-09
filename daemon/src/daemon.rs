use tokio::{io::AsyncReadExt, net::UnixListener};
use lib::defaults::{UNIX_SOCKET_PATH, API_URL};
use surf;
use anyhow::{Result, Context, anyhow};

async fn get_token(name: String, password: String) -> Result<String> {
    let _res = surf::get(format!("{API_URL}"))
        .header("name", name)
        .header("password", password)
        .send().await;
    todo!();
}

async fn execute_command(command: String, token: &String) -> Result<String> {
    todo!();
}

pub async fn start(token: Option<String>, name: Option<String>, password: Option<String>) -> Result<String> {
    let listener = UnixListener::bind(UNIX_SOCKET_PATH).context("while binding Unix Socket")?;

    let token = match token {
        Some(token) => token,
        None => {
            let (Some(name), Some(password)) = (name, password) else {
                return Err(anyhow!("Can't start waff_daemon without specified token or name and password"));
            };
            
            get_token(name, password).await?
        }
    };


    log::info!("waff_daemon started.");
    while let Ok((mut stream, _)) = listener.accept().await {
        loop {
            let mut command = String::new();
            let len = stream.read_to_string(&mut command).await.context("while trying to read UnixStream")?;
            log::trace!("Readed message {command} with len {len}");
            match execute_command(command, &token).await {
                Ok(res) => {
                    log::info!("{res}");
                },
                Err(err) => {
                    log::error!("{err}");
                }
            }
        }
    }
    Ok("waff_daemon stopped.".to_string())
}
