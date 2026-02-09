use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum Command {
    GetInstance {
        contest: String,
    },
    Submit {
        contest: String,
        task: String,
        code: String,
    }
}
