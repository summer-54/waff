use lib::defaults::UNIX_SOCKET_PATH;
use tokio::{io::{AsyncReadExt, AsyncWriteExt}, net::UnixStream};
use anyhow::{Result, Context};

pub async fn send_command(command: lib::command::Command) -> Result<Box<str>> {
    let mut stream = UnixStream::connect(UNIX_SOCKET_PATH).await.context("while connecting to Unix Socket")?;
    stream.write(serde_json::to_string(&command)?.as_bytes()).await.context("while writing message to Unix Socket")?;
    let mut res = String::new();
    stream.read_to_string(&mut res).await?;
    Ok(res.into())
}
