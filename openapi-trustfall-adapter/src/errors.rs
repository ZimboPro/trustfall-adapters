use std::path::PathBuf;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum OpenAPIAdapterErrors {
    #[error("Path is not a directory {0}")]
    PathIsNotADirectory(PathBuf),
    #[error("Path doesn't exist {0}")]
    PathDoesNotExist(PathBuf),
    #[error("Failed to find YAML files: {0}")]
    FilesNotFound(PathBuf),
    #[error("Failed to merge YAML files: {0}")]
    FailedToMerge(String),
    #[error("Failed to open file")]
    FailedToOpenFile(#[from] std::io::Error),
    #[error("Failed to serialize to an OpenAPI struct")]
    FailedToSerializeToOpenAPI(#[from] serde_yaml::Error),
}
