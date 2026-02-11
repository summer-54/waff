use serde::{Deserialize, Serialize};

use crate::contest_id::ContestId;

#[derive(Serialize, Deserialize)]
pub enum Command {
    GetInstance {
        contest: ContestId,
    },
    Submit {
        contest: ContestId,
        task: Box<str>,
        code: Box<str>,
    }
}
