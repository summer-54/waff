use std::{path::Path, process::Stdio};
use futures::future::join_all;
use anyhow::{Result, Context};
use tokio::{io::AsyncWriteExt, process::Command};
use lib::defaults;
use lib::instance::{
    task::Task,
    test_verdict::TestVerdict,
    test::Test,
    verdict::Verdict,
};


pub async fn run_on_input(exec_path: &Path, input: &str) -> Result<Box<str>> {
    let mut command = Command::new(format!("./{}", exec_path.to_str().unwrap_or("").to_string())).stdin(Stdio::piped()).stdout(Stdio::piped()).spawn().context("program runing")?;
    command.stdin.as_mut().unwrap().write(input.as_bytes()).await.context("writing at stdin in program")?;

    Ok(String::from_utf8(command.wait_with_output().await.context("waiting with output")?.stdout.to_vec()).context("output parsing to string")?.into())
}

pub async fn check_on_samples(litera: &str, path: &Path) -> Result<Vec<TestVerdict>> {
    let task = Task::get_from_save(&defaults::default_task_path(&litera)).await.context("Can't get task {litera} from save : {err}")?;
    let tests_verdicts: Vec<TestVerdict> = join_all(task.samples.iter()
        .map(
            |Test { input, output, .. }| async {
                let solve_output = match run_on_input(path, input).await {
                    Ok(output) => output,
                    Err(err) => {
                        return TestVerdict {
                            time: 0.0,
                            memory: 0,
                            verdict: Verdict::RE,
                            correct_output: None,
                            output: format!("{err}").into(),
                        };
                    }
                };
                if let Some(out) = output.clone() {
                    let out = { 
                        let words: Vec<_> = out.split_whitespace().collect();
                        words.join(" ").into()
                    };
                    let solve_output = { 
                        let words: Vec<_> = solve_output.split_whitespace().collect();
                        words.join(" ").into()
                    };
                    if out == solve_output {
                        TestVerdict {
                            time: 0.0,
                            memory: 0,
                            verdict: Verdict::OK,
                            output: solve_output,
                            correct_output: None,
                        }
                    } else {
                        TestVerdict {
                            time: 0.0,
                            memory: 0,
                            verdict: Verdict::WA,
                            output: solve_output,
                            correct_output: Some(out),
                        }
                    }
                } else {
                    TestVerdict {
                        time: 0.0,
                        memory: 0,
                        verdict: Verdict::OKWA,
                        output: solve_output,
                        correct_output: None,
                    }
                }
            }
        )
    ).await;
    Ok(tests_verdicts)
}
