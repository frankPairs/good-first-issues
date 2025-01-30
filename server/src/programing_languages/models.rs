use serde::Serialize;
use std::fmt;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum ProgrammingLanguage {
    Rust,
    Java,
    Javascript,
    Go,
    Python,
    Ruby,
}

impl fmt::Display for ProgrammingLanguage {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ProgrammingLanguage::Rust => write!(f, "Rust"),
            ProgrammingLanguage::Java => write!(f, "Java"),
            ProgrammingLanguage::Javascript => write!(f, "Javascript"),
            ProgrammingLanguage::Go => write!(f, "Go"),
            ProgrammingLanguage::Python => write!(f, "Python"),
            ProgrammingLanguage::Ruby => write!(f, "Ruby"),
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct ProgrammingLanguageResponse {
    pub id: ProgrammingLanguage,
    pub name: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct GetProgrammingLanguagesResponse {
    pub items: Vec<ProgrammingLanguageResponse>,
}
