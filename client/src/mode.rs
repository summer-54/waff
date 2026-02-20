use serde::{Serialize, Deserialize};

#[derive(clap::ValueEnum ,Debug, Deserialize, Serialize, Clone)]
pub enum Mode {
    #[value(alias("json"), alias("api"), alias("object"))]
    Json,
    #[value(alias("nvim"), alias("md"), alias("vui"))]
    Nvim,
    #[value(alias("normal"), alias("none"), alias("void"))]
    None,
}
