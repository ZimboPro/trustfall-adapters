use std::{ffi::OsStr, io::Read, path::PathBuf};

use openapiv3::{Operation, PathItem};
use serde::{Deserialize, Serialize};
use yaml_hash::YamlHash;

use crate::errors::OpenAPIAdapterErrors;

pub(crate) fn merge(files: Vec<PathBuf>) -> Result<String, OpenAPIAdapterErrors> {
    let mut hash = YamlHash::new();
    for file in files {
        if !(file.exists() && file.is_file()) {
            return Err(OpenAPIAdapterErrors::PathDoesNotExist(file));
        }
        let contents = open_file(file)?;
        hash = hash
            .merge_str(&contents)
            .map_err(|e| OpenAPIAdapterErrors::FailedToMerge(e.to_string()))?;
    }

    Ok(hash.to_string())
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Route {
    pub path: String,
    pub get: Option<Operator>,
    pub post: Option<Operator>,
    pub put: Option<Operator>,
    pub delete: Option<Operator>,
    pub patch: Option<Operator>,
    pub options: Option<Operator>,
}

impl From<openapiv3::ReferenceOr<PathItem>> for Route {
    fn from(value: openapiv3::ReferenceOr<PathItem>) -> Self {
        match value {
            openapiv3::ReferenceOr::Reference { reference: _ } => todo!("Implement reference path"),
            openapiv3::ReferenceOr::Item(item) => Self {
                path: "".to_string(),
                get: item.get.map(|x| Operator::from_operation(&x, "GET")),
                post: item.post.map(|x| Operator::from_operation(&x, "POST")),
                put: item.put.map(|x| Operator::from_operation(&x, "PUT")),
                delete: item.delete.map(|x| Operator::from_operation(&x, "DELETE")),
                patch: item.patch.map(|x| Operator::from_operation(&x, "PATCH")),
                options: item
                    .options
                    .map(|x| Operator::from_operation(&x, "OPTIONS")),
            },
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Operator {
    pub method: String,
    pub summary: Option<String>,
    pub description: Option<String>,
    pub tags: Option<Vec<String>>,
    pub aws: Option<AmazonApigatewayIntegration>,
}

impl Operator {
    pub fn from_operation(operation: &Operation, method: &str) -> Self {
        Self {
            method: method.to_string(),
            summary: operation.summary.clone(),
            description: operation.description.clone(),
            tags: if operation.tags.is_empty() {
                None
            } else {
                Some(operation.tags.clone())
            },
            aws: match operation.extensions.get("x-amazon-apigateway-integration") {
                Some(value) => {
                    match serde_json::from_value::<AmazonApigatewayIntegration>(value.clone()) {
                        Ok(mut s) => {
                            s.extract_supplementary_data();
                            Some(s)
                        }
                        Err(e) => {
                            eprintln!("Failed to deserialize to AWS extension: {e} {value}");
                            None
                        }
                    }
                }
                None => None,
            },
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AmazonApigatewayIntegration {
    #[serde(rename = "type")]
    pub r_type: String,
    pub http_method: String,
    pub uri: String,
    #[serde(rename = "passthroughBehavior")]
    pub pass_through_behavior: String,
    pub timeout_in_millis: Option<usize>,
    #[serde(skip)]
    pub trigger: String,
    #[serde(skip)]
    pub arn: String,
}

impl AmazonApigatewayIntegration {
    pub fn extract_supplementary_data(&mut self) {
        let splits: Vec<&str> = self.uri.split(':').collect();
        self.trigger = match splits[4] {
            "lambda" => {
                let x = splits.last().unwrap().split_once('{').unwrap();
                self.arn = x.1.split_once('}').unwrap().0.to_string();
                "Lambda".to_owned()
            }
            "state" => "Step Function".to_owned(),
            x => x.to_owned(),
        };
    }
}

pub(crate) fn find_files(path: &std::path::Path, extension: &OsStr) -> Vec<PathBuf> {
    let mut files = Vec::new();
    for entry in path.read_dir().expect("Failed to read directory").flatten() {
        if entry.path().is_dir() {
            files.append(&mut find_files(&entry.path(), extension));
        } else if entry.path().extension() == Some(extension) {
            files.push(entry.path());
        }
    }
    files
}

/// Gets a file's contents
pub(crate) fn open_file(filename: PathBuf) -> Result<String, OpenAPIAdapterErrors> {
    let mut file =
        std::fs::File::open(filename).map_err(|e| OpenAPIAdapterErrors::FailedToOpenFile(e))?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Couldn't read the contents of the file");
    Ok(contents)
}
