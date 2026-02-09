use crate::instance::test_verdict::TestVerdict;

pub fn format_tests_verdicts(verdicts: Vec<TestVerdict>) -> String {
    let mut ans = String::new();
    for verdict in verdicts {
        ans += &format!("{:?}\n{}\n", verdict.verdict, verdict.output);
        if let Some(co) = verdict.correct_output {
            ans += &format!("⮤your⮥|⮦correct⮧\n{}\n-=-=-=-=-", co);
        } else {
            ans += &format!("===\n");
        }
    }
    ans
}
