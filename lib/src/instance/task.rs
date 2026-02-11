use std::{collections::HashMap};

use crate::{defaults, ts_api};
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use tokio::{
    fs::{DirBuilder, File, read_dir},
    io::{AsyncReadExt, AsyncWriteExt},
};

use super::test::Test;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Info {
    pub name: Box<str>,
    pub id: i32,
    pub litera: Box<str>,
    pub time_limit: f32,
    pub memory_limit: i32,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Task {
    pub info: Info,
    pub statements: HashMap<Box<str>, Box<str>>,
    pub samples: Vec<Test>,
}

impl Task {
    pub async fn save_to(&self, path: &str) -> Result<()> {
        let dir_builder = DirBuilder::new();
        dir_builder
            .create(path.clone())
            .await
            .context("while creating Task folder")?;

        let mut task_file = File::create(&*defaults::info_path(path))
            .await
            .context("while cerating Task file")?;
        task_file
            .write(
                serde_json::to_string(&self.info)
                    .context("while parsing task to json")?
                    .as_bytes(),
            )
            .await
            .context("while writing to Task file")?;

        dir_builder
            .create(&*defaults::statements_path(path))
            .await
            .context("while creating Statements folder")?;
        for (file_name, statement) in &self.statements {
            let mut statement_file = File::create(&*defaults::statement_path(
                &defaults::statements_path(path),
                file_name,
            ))
            .await
            .context("while creating statement file")?;
            statement_file
                .write(statement.as_bytes())
                .await
                .context("while writing to statement file")?;
        }

        dir_builder
            .create(&*defaults::samples_path(path))
            .await
            .context("while creating samples directory")?;

        let mut samples = self.samples.clone();
        // Sorting by `number`
        samples.sort_by(|a, b| a.number.cmp(&b.number));

        for sample in samples {
            sample
                .save_to(
                    &defaults::input_path(&*defaults::samples_path(path), sample.number),
                    &defaults::output_path(&*defaults::samples_path(path), sample.number),
                )
                .await?;
        }
        Ok(())
    }

    pub async fn get_from_save(path: &str) -> Result<Self> {
        let info = {
            let mut task_file = File::open(&*defaults::info_path(path))
                .await
                .context("while opening info file")?;
            let mut info_string = String::new();
            task_file
                .read_to_string(&mut info_string)
                .await
                .context("while reading info file")?;
            serde_json::from_str(&info_string).context("while parsing json to `Info`")?
        };

        // Reading `statements` folder :

        let mut statements = HashMap::<Box<str>, Box<str>>::new();
        {
            let mut statement_files = read_dir(&*defaults::statements_path(path))
                .await
                .context("while reading statements directory")?;
            while let Ok(Some(dir_entry)) = statement_files.next_entry().await {
                log::trace!("Reading {:?} statement", dir_entry.path().to_str());

                let mut statement_file = File::open(dir_entry.path())
                    .await
                    .context("whiler opening statements file")?;
                let mut statement = String::new();
                statement_file
                    .read_to_string(&mut statement)
                    .await
                    .context("while reading statements file")?;

                if let Ok(file_name) = dir_entry.file_name().into_string() {
                    statements.insert(file_name.into(), statement.into());
                }
            }
        }

        // Reading `samples` folder :

        let mut samples = Vec::<Test>::new();
        let mut test_number = 1;
        while let Ok(test) = Test::get_from_save(
            test_number,
            &defaults::input_path(&defaults::samples_path(path), test_number),
            &defaults::output_path(&defaults::samples_path(path), test_number),
        )
        .await
        {
            log::trace!("Reading {} test", test.number);
            samples.push(test);
            test_number += 1;
        }

        // Sorting by `number`
        samples.sort_by(|a, b| a.number.cmp(&b.number));

        Ok(Self {
            info,
            samples,
            statements,
        })
    }

    pub fn from_api(index: u32, task: ts_api::Task) -> Self {
        let A = 0x41;
        let litera = match index {
            i if i < 26 => {
                String::from(char::from_u32(A + i).unwrap_or('?'))
            },
            i if i >= 26 => {
                (i - 25).to_string()
            },
            _ => String::from('?').into(),
        }.into();
        Self {
            info: Info {
                litera,
                id: task.id,
                memory_limit: task.ml,
                time_limit: task.tl as f32 / 1000.0,
                name: task.name,
            },
            samples: vec![

            ],
            statements: HashMap::new(),
        }
    }
}
