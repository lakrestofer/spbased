use super::*;

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    models: Vec<Model>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Model {
    name: String,
    program: String,
    description: Option<String>,
}

impl Model {
    pub fn new(name: &str, program: &str) -> Self {
        Self {
            name: name.into(),
            program: program.into(),
            description: None,
        }
    }
}
impl Default for Config {
    fn default() -> Self {
        Self { models: Vec::new() }
    }
}
