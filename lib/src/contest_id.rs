use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ContestId {
    pub contest: i32,
    pub group: i32,
}
