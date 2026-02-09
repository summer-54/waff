use crate::defaults::UNIX_SOCKET_PATH;
use tokio::{io::{AsyncReadExt, AsyncWriteExt}, net::UnixStream, fs::File};
use crate::Instance;
use crate::command::Command;
use anyhow::{Result, Context};

pub async fn submit(contest: String, task: String, file: &mut File) -> Result<String> {
    let mut code = String::new();
    file.read_to_string(&mut code).await.context("while reading submitted file")?; 
    let mut stream = UnixStream::connect(UNIX_SOCKET_PATH).await.context("while connecting to Unix Socket")?;
    stream.write(serde_json::to_string(&Command::Submit {contest: contest.clone(), task: task.clone(), code: code.clone()})?.as_bytes()).await.context("while writing SUBMIT message")?;
    log::trace!("Sended submission to contest {contest} on task {task}. Code:\n{code}");
    Ok(format!("Your submission to {task} sended.").to_string())
}


pub async fn download_instance(contest: String) -> Result<Instance> {
    let mut stream = UnixStream::connect(UNIX_SOCKET_PATH).await.context("while connecting to Unix Socket")?;
    stream.write(serde_json::to_string(&Command::GetInstance { contest })?.as_bytes()).await.context("while writing GetInstance to daemon")?;
    let mut instance_string = String::new();
    stream.read_to_string(&mut instance_string).await.context("while trying to read instance from daemon");

    Ok(Instance::from_api_json(serde_json::Value::from(instance_string)).context("while trying to parse instance in structure")?)
}
