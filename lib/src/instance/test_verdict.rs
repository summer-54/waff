use super::Verdict;

#[derive(Debug)]
pub struct TestVerdict {
    pub verdict: Verdict,
    pub correct_output: Option<String>,
    pub time: f32,
    pub memory: u32,
    pub output: String,
}
