use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use tokio::{
    fs::File,
    io::{AsyncReadExt, AsyncWriteExt},
};

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Contest {
    pub id: Box<str>,
    pub tasks: Vec<Box<str>>,
}

impl Contest {
    pub async fn save_to(&self, path: &str) -> Result<()> {
        let mut file = File::create(path)
            .await
            .context("while creating file Contest file")?;
        file.write(
            serde_json::to_string(&self)
                .context("while parsing Contest to json")?
                .as_bytes(),
        )
        .await
        .context("while writing in Contest file")?;
        Ok(())
    }
    pub async fn get_from_save(path: &str) -> Result<Self> {
        let mut file = File::open(path)
            .await
            .context("while opening Contest file")?;
        let mut string = String::new();
        file.read_to_string(&mut string)
            .await
            .context("while reading Contest file")?;
        Ok(serde_json::from_str(&string).context("while parsing json to Contest")?)
    }
}
