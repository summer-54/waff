use contest_id::ContestId;
use instance::Instance;
use token::Token;
use surf;
use crate::prelude::*;

pub async fn get_token(name: &str, password: &str) -> anyhow::Result<Token> {
    todo!();
    let res = surf::get(format!("{API_URL}"))
        .header("name", name)
        .header("password", password)
        .send().await.map_err(|e| anyhow!("{e}"))?
        .body_string().await.map_err(|e| anyhow!("{e}"))?;
}

pub async fn get_contest(token: &Token, contest_id: &ContestId) -> anyhow::Result<Instance> {
    let ContestId {group, contest} = contest_id; 
    let res = surf::get(format!("{API_URL}/get_contest_info?contestId={contest}&groupId={group}"))
        .header("Authorization", &**token)
        .send().await.map_err(|e| anyhow!("{e}"))?
        .body_string().await.map_err(|e| anyhow!("{e}"))?;

    let contest: ts_api::ContestWithTasks = serde_json::from_str(&res)?;
    let instance = Instance::from_api(contest);
    Ok(instance)
}
