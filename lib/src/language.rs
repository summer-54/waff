use serde::{Deserialize, Serialize};

#[derive(clap::ValueEnum ,Debug, Deserialize, Serialize, Clone)]
pub enum Language {
    #[value(alias("cpp"), alias("c++"), alias("g++"), alias("gcc"), alias("mingw"))]
    Cpp,
    #[value(alias("py"), alias("python3"), alias("python"), alias("pypy3"))]
    Py,
}

impl Language {
    pub fn to_api_str(&self) -> Box<str> {
        match self {
            Self::Cpp => "cpp",
            Self::Py => "py",
        }.into()
    }
}
