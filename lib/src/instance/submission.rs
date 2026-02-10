use super::{TestVerdict, Verdict};
use std::collections::HashMap;

pub struct Submission {
    code: Box<str>,
    id: u32,
    test_verdicts: HashMap<u32, TestVerdict>,
    verdict: Verdict,
}
