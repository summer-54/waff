use lib::{instance::test_verdict::TestVerdict, ts_api};

use crate::mode::Mode;

pub fn format_tests_verdicts(verdicts: Vec<TestVerdict>, mode: Mode) -> Box<str> {
    match mode {
        Mode::None => {
            let mut ans = String::new();
            for verdict in verdicts {
                ans += &format!("{:?}\n{}\n", verdict.verdict, verdict.output);
                if let Some(co) = verdict.correct_output {
                    ans += &format!("⮤your⮥|⮦correct⮧\n{}\n-=-=-=-=-", co);
                } else {
                    ans += &format!("===\n");
                }
            }
            ans.into()
        },
        Mode::Json => {
            serde_json::to_string(&verdicts).unwrap_or("{}".to_string()).into()
        },
        Mode::Nvim => {
            let mut ans = String::new();
            for verdict in verdicts {
                ans += &format!("`{:?}`
{}:w
", verdict.verdict, verdict.output);
                if let Some(co) = verdict.correct_output {
                    ans += &format!("⮤your⮥|⮦correct⮧\n{}\n-=-=-=-=-", co);
                } else {
                    ans += &format!("===\n");
                }
            }
            ans.into()
        },
    }
}


/*

# daffas



*/

pub fn format_submission_status(submission: ts_api::FullSubmission, mode: Mode) -> Box<str> {
    match mode {
        Mode::Nvim => format!("*submission* `{}` -- by `{} ({})` -- *task* `{} ({})` in *contest* `{}`
*verdict* `{}` on *test* `{}` at `{}`

``` {}
{}
```", submission.id, submission.author_name, submission.user_id, submission.task_name, submission.task_id, submission.contest_id, submission.verdict, submission.test, submission.time, submission.language, submission.source_code).into(),
        Mode::Json => serde_json::to_string(&submission).unwrap_or("{}".to_string()).into(),    
        Mode::None => format!("submission {} -- by {} ({}) -- task {} ({}) in contest {}
verdict {} on test {} at {} with language {}

{}
", submission.id, submission.author_name, submission.user_id, submission.task_name, submission.task_id, submission.contest_id, submission.verdict, submission.test, submission.time, submission.language, submission.source_code).into(),
    }
}
