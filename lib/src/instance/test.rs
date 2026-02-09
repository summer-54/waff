use serde::{Serialize, Deserialize};
use tokio::{fs::File, io::{AsyncReadExt, AsyncWriteExt}};
use anyhow::{Result, Context};

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Test {
    pub number: u32,
    pub input: String,
    pub output: Option<String>,
}

impl Test {
    pub async fn save_to(&self, in_path: &String, out_path: &String) -> Result<()> {
        let mut input_file = File::create(in_path.clone()).await.context("while creating input file")?;
        input_file.write(self.input.as_bytes()).await.context("while writing in input file")?;
        if let Some(output) = &self.output {
            let mut output_file = File::create(out_path.clone()).await.context("while creating output file")?;
            output_file.write(output.as_bytes()).await.context("while writing in output file")?;
        }
        Ok(())
    }
    pub async fn get_from_save(number: u32, in_path: &String, out_path: &String) -> Result<Self> {
        let mut input_file = File::open(in_path.clone()).await.context("while opening input file")?;
        let mut input = String::new();
        input_file.read_to_string(&mut input).await.context("while reading input file")?;
        let output = {
            if let Ok(mut output_file) = File::open(out_path.clone()).await {
                let mut output = String::new();
                output_file.read_to_string(&mut output).await.context("while reading output file")?;
                Some(output)
            } else {
                None
            }
        };
        Ok( Self {
            number,
            input,
            output
        } )
    }
}

