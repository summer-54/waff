use lib::defaults::UNIX_SOCKET_PATH;
use tokio::{io::{AsyncReadExt, AsyncWriteExt}, net::UnixStream, fs::File};
use lib::instance::Instance;
use lib::command::Command;
use anyhow::{Result, Context};

pub async fn submit(contest: &str, task: &str, file: &mut File) -> Result<Box<str>> {
    let mut code = String::new();
    file.read_to_string(&mut code).await.context("while reading submitted file")?; 
    let mut stream = UnixStream::connect(UNIX_SOCKET_PATH).await.context("while connecting to Unix Socket")?;
    stream.write(serde_json::to_string(&Command::Submit {
        contest: contest.into(), task: task.into(), code: code.clone().into(),
    })?.as_bytes()).await.context("while writing SUBMIT message")?;
    log::trace!("Sended submission to contest {contest} on task {task}. Code:\n{code}");
    Ok(format!("Your submission to {task} sended.").into())
}


pub async fn download_instance(contest: &str) -> Result<Instance> {
    let mut stream = UnixStream::connect(UNIX_SOCKET_PATH).await.context("while connecting to Unix Socket")?;
    stream.write(serde_json::to_string(&Command::GetInstance { contest: contest.into() })?.as_bytes()).await.context("while writing GetInstance to daemon")?;
    let mut instance_string = String::new();
    stream.read_to_string(&mut instance_string).await.context("while trying to read instance from daemon")?;

    Ok(Instance::from_api_json(serde_json::Value::from(instance_string)).context("while trying to parse instance in structure")?)
}
