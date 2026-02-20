use serde::{Deserialize, Serialize};

use crate::contest_id::ContestId;
use crate::language::Language;

#[derive(Serialize, Deserialize, Debug)]
pub enum Command {
    GetInstance {
        contest: ContestId,
    },
    GetSubmissionStatus {
        submission_id: i32,
    },
    Submit {
        language: Language,
        contest_id: ContestId,
        task_id: i32,
        code: Box<str>,
    }
}
