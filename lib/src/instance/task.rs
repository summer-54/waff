use std::collections::HashMap;

use serde::{Serialize, Deserialize};
use tokio::{fs::{read_dir, DirBuilder, File}, io::{AsyncReadExt, AsyncWriteExt}};
use anyhow::{Result, Context};
use crate::defaults;

use super::test::Test;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Info {
    pub name: String,
    pub litera: String,
    pub time_limit: f32,
    pub memory_limit: u64,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Task {
    pub info: Info,
    pub statements: HashMap<String, String>,
    pub samples: Vec<Test>,
}

impl Task {
    pub async fn save_to(&self, path: &String) -> Result<()> {
        let dir_builder = DirBuilder::new();
        dir_builder.create(path.clone()).await.context("while creating Task folder")?;
        
        let mut task_file = File::create(defaults::info_path(path)).await.context("while cerating Task file")?;
        task_file.write(
            serde_json::to_string(&self.info).context("while parsing task to json")?.as_bytes()
        ).await.context("while writing to Task file")?;

        dir_builder.create(defaults::statements_path(path)).await.context("while creating Statements folder")?;
        for (file_name, statement) in &self.statements {
            let mut statement_file = File::create(defaults::statement_path(&defaults::statements_path(path), file_name)).await.context("while creating statement file")?;
            statement_file.write(statement.as_bytes()).await.context("while writing to statement file")?;
        }

        dir_builder.create(defaults::samples_path(path)).await.context("while creating samples directory")?;

        let mut samples = self.samples.clone();
        // Sorting by `number`
        samples.sort_by(|a, b| a.number.cmp(&b.number));

        for sample in samples {
            sample.save_to(
                &defaults::input_path(&defaults::samples_path(path), sample.number),
                &defaults::output_path(&defaults::samples_path(path), sample.number),
            ).await?;
        }
        Ok(())
    }

    pub async fn get_from_save(path: &String) -> Result<Self> {
        let info = {
            let mut task_file = File::open(defaults::info_path(path)).await.context("while opening info file")?;
            let mut info_string = String::new();
            task_file.read_to_string(&mut info_string).await.context("while reading info file")?;
            serde_json::from_str(&info_string).context("while parsing json to `Info`")?
        };
                
        // Reading `statements` folder :

        let mut statements = HashMap::<String, String>::new();
        {
            let mut statement_files = read_dir(defaults::statements_path(path)).await.context("while reading statements directory")?;
            while let Ok(Some(dir_entry)) = statement_files.next_entry().await {
                log::trace!("Reading {:?} statement", dir_entry.path().to_str());

                let mut statement_file = File::open(dir_entry.path()).await.context("whiler opening statements file")?;
                let mut statement = String::new();
                statement_file.read_to_string(&mut statement).await.context("while reading statements file")?;

                if let Ok(file_name) = dir_entry.file_name().into_string() {
                    statements.insert(file_name, statement);
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
        ).await {
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
}

