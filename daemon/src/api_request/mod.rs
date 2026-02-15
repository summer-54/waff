use contest_id::ContestId;
use instance::Instance;
use token::Token;
use crate::prelude::*;

pub async fn get_token(_name: &str, _password: &str) -> anyhow::Result<Token> {
    Err(anyhow!("get token isn't supported now"))
}

pub async fn get_contest(token: &Token, contest_id: &ContestId) -> anyhow::Result<Instance> {
    let ContestId {group, contest} = contest_id; 
    log::error!("{}", &**token);
    let qwery = reqwest::Client::new()
        //.get(format!("https://nativerest.net/echo/get?contestId={contest}&groupId={group}"))
        .get(format!("{API_URL}/get_contest_info?contestId={contest}&groupId={group}"))
        .header("Authorization", &**token);
    log::trace!("Qwery: {qwery:?}");
    let res = qwery
        .send().await?
        .text().await?;
    log::info!("Result recieved {res}.");
    let mut contest: ts_api::ContestWithTasks = serde_json::from_str(&res)?;
    contest.group_id = Some(contest_id.group);
    let instance = Instance::from_api(contest)?;
    Ok(instance)
}
