use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use tokio::{
    fs::File,
    io::{AsyncReadExt, AsyncWriteExt},
};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Test {
    pub number: u32,
    pub input: Box<str>,
    pub output: Option<Box<str>>,
}

impl Test {
    pub async fn save_to(&self, in_path: &str, out_path: &str) -> Result<()> {
        let mut input_file = File::create(in_path)
            .await
            .context("while creating input file")?;
        input_file
            .write(self.input.as_bytes())
            .await
            .context("while writing in input file")?;
        if let Some(output) = &self.output {
            let mut output_file = File::create(out_path)
                .await
                .context("while creating output file")?;
            output_file
                .write(output.as_bytes())
                .await
                .context("while writing in output file")?;
        }
        Ok(())
    }
    pub async fn get_from_save(number: u32, in_path: &str, out_path: &str) -> Result<Self> {
        let mut input_file = File::open(in_path)
            .await
            .context("while opening input file")?;
        let mut input = String::new();
        input_file
            .read_to_string(&mut input)
            .await
            .context("while reading input file")?;
        let output = {
            if let Ok(mut output_file) = File::open(out_path).await {
                let mut output = String::new();
                output_file
                    .read_to_string(&mut output)
                    .await
                    .context("while reading output file")?;
                Some(output.into())
            } else {
                None
            }
        };
        Ok(Self {
            number,
            input: input.into(),
            output: output,
        })
    }
}
