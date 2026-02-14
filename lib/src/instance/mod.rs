pub mod contest;
pub mod submission;
pub mod task;
pub mod test;
pub mod test_verdict;
pub mod verdict;

use crate::{defaults, ts_api, contest_id::ContestId};
use anyhow::{Context, Result, anyhow};
use contest::Contest;
use task::Task;
use test_verdict::TestVerdict;
use tokio::{
    fs::{DirBuilder, File, read_dir, remove_dir_all},
    io::AsyncReadExt,
};
use verdict::Verdict;

#[derive(Debug, Clone)]
pub struct Instance {
    contest: Contest,
    tasks: Vec<Task>,
}

impl Instance {
    pub async fn save_to(self, directory: &str) -> Result<()> {
        remove_dir_all(directory)
            .await
            .context("removing save directory")?;
        let dir_builder = DirBuilder::new();
        dir_builder
            .create(directory)
            .await
            .context("creating save directory")?;
        self.contest
            .save_to(&*defaults::contest_path(directory))
            .await?;
        dir_builder
            .create(&*defaults::tasks_path(directory))
            .await
            .context("creating task directory")?;
        let mut tasks = self.tasks;
        // Sorting by `litera`
        tasks.sort_by(|a, b| a.info.litera.cmp(&b.info.litera));

        for task in tasks {
            let litera = &*task.info.litera;
            task.save_to(&*defaults::task_path(
                &*defaults::tasks_path(directory),
                &*litera,
            ))
            .await?;
        }
        Ok(())
    }

    pub async fn get_from_dir(directory: &str) -> Result<Self> {
        let contest = Contest::get_from_save(&defaults::contest_path(directory)).await?;
        let mut tasks = Vec::new();
        let mut taskfiles = read_dir(&*defaults::tasks_path(directory))
            .await
            .context("reading from task directory")?;
        while let Ok(Some(dir_entry)) = taskfiles.next_entry().await {
            tasks.push(Task::get_from_save(&dir_entry.path().to_string_lossy().to_string()).await?);
        }

        // Sorting by `litera`
        tasks.sort_by(|a, b| a.info.litera.cmp(&b.info.litera));

        Ok(Self { contest, tasks })
    }

    pub fn from_api(contest: ts_api::ContestWithTasks) -> Result<Self> {
        let Some(group_id) = contest.group_id else {
            return Err(anyhow!("Group id isn't specified"));
        };
        let tasks: Vec<Task> = contest.tasks.to_vec().into_iter().enumerate().map(|(index, task_api)| Task::from_api(index as u32, task_api)).collect();
        Ok(Self {
            contest: Contest {
                id: ContestId {
                    contest: contest.id,
                    group: group_id,
                },
                tasks: tasks.iter()
                    .map(|task| task.info.litera.clone())
                    .collect(),
            },
            tasks,
        })
    }
}

async fn read_to_json(file: &mut File) -> Result<serde_json::Value> {
    let mut text = String::new();
    let len = file
        .read_to_string(&mut text)
        .await
        .context("while read_to_json file")?;
    log::trace!("File was readed with len {len}:\n{text}");
    let json: serde_json::Value =
        serde_json::from_str(&text).context("while parsing file to json")?;
    Ok(json)
}

pub async fn get_contest_id(instance_path: &str) -> Result<ContestId> {
    let instance_path = format!("{instance_path}/contest.json");
    let mut contest_file = File::open(&instance_path)
        .await
        .context("while reading contest file")?;
    let json = read_to_json(&mut contest_file).await?;

    let contest: Contest =
        serde_json::from_value(json).context("while parsing Contest from json")?;
    Ok(contest.id)
}

#[tokio::test]
pub async fn instance_save_to_and_read_from() {
    let mut st = std::collections::HashMap::new();
    st.insert("eo.md".into(), "Statements\nin file eo.md".into());
    let mut st2 = std::collections::HashMap::new();
    st2.insert(
        "eo.md".into(),
        "Statements of second task\nin file eo.md".into(),
    );
    let instance = Instance {
        contest: Contest {
            id: ContestId {
                group: -1,
                contest: 54,
            },
            tasks: vec!["A".into(), "B".into()],
        },
        tasks: vec![
            Task {
                info: task::Info {
                    id: 7,
                    time_limit: 1.0,
                    memory_limit: 256000,
                    litera: "A".into(),
                    name: "Sum of two number".into(),
                },
                samples: vec![
                    test::Test {
                        number: 1,
                        input: "2 2".into(),
                        output: Some("4".into()),
                    },
                    test::Test {
                        number: 2,
                        input: "2 3".into(),
                        output: Some("5".into()),
                    },
                ],
                statements: st,
            },
            Task {
                info: task::Info {
                    id: 171,
                    time_limit: 1.0,
                    memory_limit: 256000,
                    litera: "B".into(),
                    name: "Diff btw two number".into(),
                },
                samples: vec![
                    test::Test {
                        number: 1,
                        input: "2 2".into(),
                        output: Some("0".into()),
                    },
                    test::Test {
                        number: 2,
                        input: "2 3".into(),
                        output: Some("1".into()),
                    },
                ],
                statements: st2,
            },
        ],
    };
    let res = instance.clone().save_to(".tost").await;
    if let Err(err) = res {
        panic!("Error of saving: {err:?}");
    };
    let loaded = Instance::get_from_dir(".tost").await;
    match loaded {
        Err(err) => {
            panic!("Error of get_from_dir: {err:?}");
        }
        Ok(_) => {
            //assert_eq!(instance, r);
        }
    }
}
