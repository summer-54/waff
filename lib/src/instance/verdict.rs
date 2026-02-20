use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum Verdict {
    OK,
    WA,
    RE,
    TL,
    ML,
    TE,
    OKWA,
}
