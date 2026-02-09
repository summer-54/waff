pub mod task;
pub mod test;
pub mod test_verdict;
pub mod verdict;
pub mod contest;
pub mod submission;

use serde_json::Value;
use crate::defaults;
use tokio::{fs::{read_dir, remove_dir_all, DirBuilder, File}, io::AsyncReadExt};
use verdict::Verdict;
use test_verdict::TestVerdict;
use contest::Contest;
use task::Task;
use anyhow::{Context, Result};

#[derive(Debug, PartialEq, Clone)]
pub struct Instance {
    contest: Contest,
    tasks: Vec<Task>,
}

impl Instance {
    pub async fn save_to(self, directory: &String) -> Result<()> {
        remove_dir_all(format!("{directory}")).await.context("removing save directory")?;
        let dir_builder = DirBuilder::new();
        dir_builder.create(format!("{directory}")).await.context("creating save directory")?;
        self.contest.save_to(&defaults::contest_path(directory)).await?;
        dir_builder.create(defaults::tasks_path(directory)).await.context("creating task directory")?;
        let mut tasks = self.tasks.clone();
        // Sorting by `litera`
        tasks.sort_by(|a, b| a.info.litera.cmp(&b.info.litera));
        
        for task in tasks {
            let litera = task.info.litera.clone();
            task.save_to(&defaults::task_path(&defaults::tasks_path(directory), &litera)).await?;
        }
        Ok(())
    }

    pub async fn get_from_dir(directory: &String) -> Result<Self> {
        let contest = Contest::get_from_save(&defaults::contest_path(directory)).await?;
        let mut tasks = Vec::new();
        let mut taskfiles = read_dir(defaults::tasks_path(directory)).await.context("reading from task directory")?; 
        while let Ok(Some(dir_entry)) = taskfiles.next_entry().await {
            tasks.push(Task::get_from_save(&dir_entry.path().to_string_lossy().to_string()).await?);
        }

        // Sorting by `litera`
        tasks.sort_by(|a, b| a.info.litera.cmp(&b.info.litera));

        Ok( Self {
            contest,
            tasks
        } )
    }

    pub fn from_api_json(value: Value) -> anyhow::Result<Self> {
        todo!()
    }
}

async fn read_to_json(file: &mut File) -> Result<serde_json::Value> {
    let mut text = String::new();
    let len = file.read_to_string(&mut text).await.context("while read_to_json file")?;
    log::trace!("File was readed with len {len}:\n{text}");
    let json: serde_json::Value = serde_json::from_str(&text).context("while parsing file to json")?;
    Ok(json)
}

pub async fn get_contest_id(mut instance_path: String) -> Result<String> {
    instance_path.push_str("/contest.json");
    let mut contest_file = File::open(&instance_path).await.context("while reading contest file")?;
    let json = read_to_json(&mut contest_file).await?;
    

    let contest: Contest = serde_json::from_value(json).context("while parsing Contest from json")?;
    Ok(contest.id)
}

#[tokio::test]
pub async fn instance_save_to_and_read_from() {
    let mut st = std::collections::HashMap::new();
    st.insert("eo.md".to_string(), "Statements\nin file eo.md".to_string());
    let mut st2 = std::collections::HashMap::new();
    st2.insert("eo.md".to_string(), "Statements of second task\nin file eo.md".to_string());
    let instance = Instance {
        contest: Contest {
            id: "test_contest".to_string(),
            tasks: vec![
                "A".to_string(),
                "B".to_string(),
            ],
        },
        tasks: vec![
            Task {
                info: task::Info {
                    time_limit: 1.0,
                    memory_limit: 256000,
                    litera: "A".to_string(),
                    name: "Sum of two number".to_string(),
                },
                samples: vec![
                    test::Test {
                        number: 1,
                        input: "2 2".to_string(),
                        output: Some("4".to_string()),
                    },
                    test::Test {
                        number: 2,
                        input: "2 3".to_string(),
                        output: Some("5".to_string()),
                    },
                ],
                statements: st,
            },
            Task {
                info: task::Info {
                    time_limit: 1.0,
                    memory_limit: 256000,
                    litera: "B".to_string(),
                    name: "Diff btw two number".to_string(),
                },
                samples: vec![
                    test::Test {
                        number: 1,
                        input: "2 2".to_string(),
                        output: Some("0".to_string()),
                    },
                    test::Test {
                        number: 2,
                        input: "2 3".to_string(),
                        output: Some("1".to_string()),
                    },
                ],
                statements: st2,
            },
        ],
    };
    let res = instance.clone().save_to(&".tost".to_string()).await;
    if let Err(err) = res {
        panic!("Error of saving: {err:?}");
    };
    let loaded = Instance::get_from_dir(&".tost".to_string()).await;
    match loaded {
        Err(err) => {
            panic!("Error of get_from_dir: {err:?}");
        },
        Ok(r) => {
            assert_eq!(instance, r);
        },
    }
}
