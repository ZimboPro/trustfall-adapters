use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Default, Clone)]
pub struct HCL {
    pub modules: Vec<Module>,
    pub terraform: Vec<Terraform>,
    pub api_config: Option<ApiConfig>,
    pub lambda: Vec<Lambda>,
    pub raw: Vec<Value>,
}

#[derive(Debug, Default, Clone)]
pub struct Module {
    pub name: String,
    // pub tag: Option<String>,
    pub source: String,
    pub version: String,
    pub variables: Vec<Variable>,
}

impl Module {
    pub fn from_serde(value: serde_json::Value, name: &str) -> Self {
        let source = value["source"].as_str().unwrap().to_string();
        let version = value["version"].as_str().unwrap().to_string();
        let variables = value
            .as_object()
            .unwrap()
            .iter()
            .map(|(k, v)| Variable {
                name: k.to_string(),
                value: v.to_string(),
            })
            .collect();
        Self {
            name: name.to_string(),
            source,
            version,
            variables,
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct Variable {
    pub name: String,
    pub value: String,
}

#[derive(Debug, Default, Clone)]
pub struct Terraform {
    pub required_version: Option<String>,
    pub backend: Option<Backend>,
    pub required_providers: Vec<RequiredProvider>,
}

#[derive(Debug, Default, Clone)]
pub struct Backend {
    pub name: String,
}

#[derive(Debug, Default, Clone)]
pub struct RequiredProvider {
    pub name: String,
    pub source: String,
    pub version: String,
}

#[derive(Debug, Default, Clone)]
pub struct Lambda {
    pub name: String,
    pub description: String,
    pub handler: String,
    pub permissions: Vec<Permissions>,
}

#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct Permissions {
    pub statement_id: String,
    pub principal: String,
    pub source_arn: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_method: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_path: Option<String>,
}

#[derive(Debug, Default, Clone)]
pub struct ApiConfig {
    pub source: String,
    pub version: String,
    pub template_file: String,
    pub template_variables: Vec<TemplateVariable>,
}

#[derive(Debug, Default, Clone)]
pub struct TemplateVariable {
    pub name: String,
    pub value: String,
    pub lambda: Option<Lambda>,
}
