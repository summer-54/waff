use contest_id::ContestId;
use instance::Instance;
use lib::language::Language;
use token::Token;
use crate::prelude::*;

pub async fn get_token(_name: &str, _password: &str) -> anyhow::Result<Token> {
    Err(anyhow!("get token isn't supported now"))
}

pub async fn get_contest(token: &Token, contest_id: &ContestId) -> anyhow::Result<Instance> {
    let ContestId {group, contest} = contest_id; 
    let res = reqwest::Client::new()
        .get(format!("{API_URL}/get_contest_info?contestId={contest}&groupId={group}"))
        .header("Authorization", &**token)
        .send().await?
        .text().await?;
    log::info!("Result recieved {res}.");
    let mut contest: ts_api::ContestWithTasks = serde_json::from_str(&res)?;
    contest.group_id = Some(contest_id.group);
    let instance = Instance::from_api(contest)?;
    Ok(instance)
}

pub async fn submit(token: &Token, contest_id: &ContestId, task_id: i32, code: &str, language: &Language) -> anyhow::Result<Box<str>> {
    let res = reqwest::Client::new()
        .post(format!("{API_URL}/submit"))
        .header("Authorization", &**token)
        .body(serde_json::to_string(&ts_api::Submission {
            time: std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH)?.as_secs() as i64,
            contest_id: contest_id.contest,
            group_id: contest_id.group,
            task_id,
            source_code: code.into(),
            language: language.to_api_str(),
        })?)
        .send().await?;
    
    Ok(format!("Submission: {}", res.text().await?).into())
}
