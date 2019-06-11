use crate::{dml, dml::validator::directive::DirectiveValidator, source::*};
pub const SQLITE_SOURCE_NAME: &str = "sqlite";

pub struct SqliteSource {
    pub(super) name: String,
    pub(super) url: String,
    pub(super) documentation: Option<String>,
}

impl Source for SqliteSource {
    fn connector_type(&self) -> &str {
        SQLITE_SOURCE_NAME
    }
    fn name(&self) -> &String {
        &self.name
    }
    fn config(&self) -> std::collections::HashMap<String, String> {
        std::collections::HashMap::new()
    }
    fn url(&self) -> &String {
        &self.url
    }
    fn get_field_directives(&self) -> Vec<Box<DirectiveValidator<dml::Field>>> {
        vec![]
    }
    fn get_model_directives(&self) -> Vec<Box<DirectiveValidator<dml::Model>>> {
        vec![]
    }
    fn get_enum_directives(&self) -> Vec<Box<DirectiveValidator<dml::Enum>>> {
        vec![]
    }
    fn documentation(&self) -> &Option<String> {
        &self.documentation
    }
}
