use super::models::{
    GetProgrammingLanguagesResponse, ProgrammingLanguage, ProgrammingLanguageResponse,
};

pub struct ProgramingLanguageRepository {
    programing_languages: Vec<ProgrammingLanguageResponse>,
}

impl ProgramingLanguageRepository {
    pub fn new() -> ProgramingLanguageRepository {
        ProgramingLanguageRepository {
            programing_languages: vec![
                ProgrammingLanguageResponse {
                    id: ProgrammingLanguage::Rust,
                    name: ProgrammingLanguage::Rust.to_string(),
                },
                ProgrammingLanguageResponse {
                    id: ProgrammingLanguage::Java,
                    name: ProgrammingLanguage::Java.to_string(),
                },
                ProgrammingLanguageResponse {
                    id: ProgrammingLanguage::Javascript,
                    name: ProgrammingLanguage::Javascript.to_string(),
                },
                ProgrammingLanguageResponse {
                    id: ProgrammingLanguage::Go,
                    name: ProgrammingLanguage::Go.to_string(),
                },
                ProgrammingLanguageResponse {
                    id: ProgrammingLanguage::Python,
                    name: ProgrammingLanguage::Python.to_string(),
                },
                ProgrammingLanguageResponse {
                    id: ProgrammingLanguage::Ruby,
                    name: ProgrammingLanguage::Ruby.to_string(),
                },
            ],
        }
    }

    pub fn get(&self) -> GetProgrammingLanguagesResponse {
        GetProgrammingLanguagesResponse {
            items: self.programing_languages.clone(),
        }
    }
}
