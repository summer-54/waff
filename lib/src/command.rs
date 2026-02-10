use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum Command {
    GetInstance {
        contest: Box<str>,
    },
    Submit {
        contest: Box<str>,
        task: Box<str>,
        code: Box<str>,
    }
}
