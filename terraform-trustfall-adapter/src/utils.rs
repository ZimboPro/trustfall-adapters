use std::{
    ffi::OsStr,
    path::{Path, PathBuf},
};

use super::model::{
    ApiConfig, Backend, HclData, Lambda, Module, RequiredProvider, TemplateVariable, Terraform,
    Variable,
};

pub fn extract_data_from_hcl(path: &Path) -> HclData {
    let files = find_files_ignore_dir(path.to_path_buf(), "tf", ".terraform");
    extract_data_from_files(&files)
}

pub fn extract_data_from_files(files: &Vec<PathBuf>) -> HclData {
    let mut json = Vec::new();
    for file in files {
        let contents = std::fs::read_to_string(file).unwrap();
        let value: serde_json::Value = hcl::from_str(&contents).unwrap();
        json.push(value);
    }
    let mut hcl = HclData::default();
    hcl.modules = extract_modules(&json);
    hcl.terraform = extract_terraform(&json);
    hcl.lambda = extract_lambda(&json);
    hcl.api_config = extract_api_config(&json, hcl.lambda.clone());
    hcl
}

fn extract_modules(json: &Vec<serde_json::Value>) -> Vec<Module> {
    let mut modules = Vec::new();
    for value in json {
        if let Some(x) = value.get("module") {
            match x {
                serde_json::Value::Object(s) => modules.extend(s.iter().map(|record| {
                    Module {
                        name: record.0.to_string(),
                        source: record
                            .1
                            .get("source")
                            .expect("Source should be set")
                            .to_string(),
                        version: record
                            .1
                            .get("version")
                            .unwrap_or_else(|| panic!("Version should be set: {}", record.0))
                            .to_string(),
                        variables: value
                            .as_object()
                            .unwrap()
                            .iter()
                            .map(|(k, v)| Variable {
                                name: k.to_string(),
                                value: v.to_string(),
                            })
                            .collect(),
                    }
                })),
                _ => todo!(),
            }
        }
    }
    modules
}

fn extract_terraform(json: &Vec<serde_json::Value>) -> Vec<Terraform> {
    let mut terraform = Vec::new();
    for value in json {
        if let Some(x) = value.get("terraform") {
            match x {
                serde_json::Value::Object(s) => {
                    let required_version = s.get("required_version").map(|x| x.to_string());
                    let backend: Option<Vec<Backend>> = s.get("backend").map(|x| match x {
                        serde_json::Value::Object(s) => s
                            .iter()
                            .map(|(k, _v)| Backend {
                                name: k.to_string(),
                            })
                            .collect(),
                        _ => unreachable!(),
                    });
                    let required_providers: Option<Vec<RequiredProvider>> =
                        s.get("required_providers").map(|x| match x {
                            serde_json::Value::Object(s) => s
                                .iter()
                                .map(|(k, v)| RequiredProvider {
                                    name: k.to_string(),
                                    source: v.get("source").unwrap().to_string(),
                                    version: v.get("version").unwrap().to_string(),
                                })
                                .collect(),
                            _ => unreachable!(),
                        });

                    let backend = if let Some(backend) = backend {
                        if backend.is_empty() {
                            None
                        } else if backend.len() > 1 {
                            panic!("backend can only have one value");
                        } else {
                            Some(backend[0].clone())
                        }
                    } else {
                        None
                    };
                    terraform.push(Terraform {
                        required_version,
                        backend,
                        required_providers: required_providers.unwrap_or_default(),
                    })
                }
                _ => todo!(),
            }
        }
    }
    terraform
}

fn extract_lambda(json: &Vec<serde_json::Value>) -> Vec<Lambda> {
    let mut lambdas = Vec::new();
    for value in json {
        let l = value
            .get("locals")
            .and_then(|x| x.get("lambdas"))
            .map(|x| match x {
                serde_json::Value::Object(lambda) => lambda.iter().map(|(k, v)| Lambda {
                    name: k.to_string(),
                    description: v.get("description").unwrap().to_string(),
                    handler: v.get("handler").unwrap().to_string(),
                    permissions: Vec::new(),
                }),
                _ => unreachable!(),
            });
        if let Some(l) = l {
            lambdas.extend(l);
        }
    }
    for value in json {
        let val = value
            .get("locals")
            .and_then(|x| x.get("lambdas_permissions"));

        if let Some(lambdas_permissions) = val {
            for lambda in lambdas.iter_mut() {
                if let Some(x) = lambdas_permissions.get(lambda.name.clone()) {
                    lambda.permissions = serde_json::from_value(x.clone()).unwrap();
                }
            }
        }
    }
    for lambda in lambdas.iter_mut() {
        for permission in lambda.permissions.iter_mut() {
            if permission
                .source_arn
                .contains("module.service_api.rest_api_execution_arn")
            {
                let v = handle_api_gateway_lambda(permission.source_arn.clone());
                match v {
                    Ok(s) => {
                        permission.http_method = Some(s[0].clone());
                        permission.http_path = Some(s[1].clone());
                    }
                    Err(e) => {
                        eprintln!("Error: {}", e);
                        panic!();
                    }
                }
            }
        }
    }
    lambdas
}

fn extract_api_config(json: &[serde_json::Value], lambdas: Vec<Lambda>) -> Option<ApiConfig> {
    let s = json.iter().find_map(|x| {
        x.get("module")
            .and_then(|service| service.get("service_api"))
    });
    let api_config = s.and_then(|x| x.get("api_config"));
    if let Some(d) = api_config {
        let template_file = d.get("body").expect("body not found").as_str().unwrap();
        let sections: Vec<&str> = template_file.splitn(3, '\"').collect();
        let template_file = sections[1].to_string();
        let mut variables = sections[2].to_string();
        variables = variables.replacen(',', "", 1);
        variables = variables.replace(")}", "");
        variables = variables.replace('{', "");
        variables = variables.replace('}', "");
        let template_variables: Vec<TemplateVariable> = variables
            .split(',')
            .map(|x| {
                let parts: Vec<&str> = x.split('=').collect();
                TemplateVariable {
                    name: parts[0].trim().to_string(),
                    value: parts[1].trim().to_string(),
                    lambda: lambdas.iter().find_map(|l| {
                        let val: String = parts[1].trim().to_string();
                        if val.contains("module.lambda") {
                            let split = val.split('\"').collect::<Vec<&str>>();
                            if l.name == split[1] {
                                Some(l.clone())
                            } else {
                                None
                            }
                        } else {
                            None
                        }
                    }),
                }
            })
            .collect();
        Some(ApiConfig {
            source: s?.get("source").unwrap().to_string(),
            version: s?.get("version").unwrap().to_string(),
            template_file,
            template_variables,
        })
    } else {
        None
    }
}

fn handle_api_gateway_lambda(source_arn: String) -> anyhow::Result<Vec<String>> {
    let section = source_arn.replace('\"', "");
    let parts: Vec<String> = section.split('}').map(|x| x.to_string()).collect();
    if section.contains("/*/*/*") {
        Err(anyhow::anyhow!(
            "Unsupported route: {}. It should rather be explicit. eg. /*/GET/the/endpoint",
            section
        ))
    } else if section.contains('*') && section.matches('*').count() == 1 {
        let parts: Vec<String> = section.split('*').map(|x| x.to_string()).collect();
        let section = parts[1].replacen('/', " ", 2);
        let mut data: Vec<String> = section.trim().split(' ').map(|x| x.to_string()).collect();
        data[1] = format!("/{}", data[1].trim());
        Ok(data)
    } else if section.contains('*') && section.matches('*').count() == 2 && section.contains("/*/*")
    {
        let parts: Vec<String> = section.split("/*/*").map(|x| x.to_string()).collect();
        let section = parts[1].replacen('/', "", 1);
        let mut data: Vec<String> = section.trim().split(' ').map(|x| x.to_string()).collect();
        data.insert(0, HttpMethod::Any.to_string());
        data[1] = format!("/{}", data[1].trim());
        Ok(data)
    } else if let Some(data) = extract_api_and_method(parts[1].trim(), HttpMethod::Get) {
        Ok([data.0, data.1].to_vec())
    } else if let Some(data) = extract_api_and_method(parts[1].trim(), HttpMethod::Post) {
        Ok([data.0, data.1].into())
    } else if let Some(data) = extract_api_and_method(parts[1].trim(), HttpMethod::Put) {
        Ok([data.0, data.1].into())
    } else if let Some(data) = extract_api_and_method(parts[1].trim(), HttpMethod::Delete) {
        Ok([data.0, data.1].into())
    } else if let Some(data) = extract_api_and_method(parts[1].trim(), HttpMethod::Patch) {
        Ok([data.0, data.1].into())
    } else {
        todo!("Need to cater for {}", parts[1].trim());
    }
}

fn extract_api_and_method(line: &str, method: HttpMethod) -> Option<(String, String)> {
    if line.contains(method.to_string().to_uppercase().as_str()) {
        Some((
            method.to_string(),
            line.replace(
                format!("/{}", method.to_string().to_uppercase()).as_str(),
                "",
            ),
        ))
    } else {
        None
    }
}

pub fn find_files_ignore_dir(path: PathBuf, extension: &str, folder: &str) -> Vec<PathBuf> {
    let mut files = Vec::new();
    for entry in path
        .read_dir()
        .expect("Failed to get dir contents")
        .flatten()
    {
        if entry.path().is_dir() && !entry.path().ends_with(folder) {
            files.extend(find_files_ignore_dir(entry.path(), extension, folder));
        } else if entry.path().is_file() && entry.path().extension() == Some(OsStr::new(extension))
        {
            files.push(entry.path().clone());
        }
    }
    files
}

/// HTTP methods
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Default, Clone)]
pub enum HttpMethod {
    #[default]
    Get,
    Post,
    Put,
    Delete,
    Patch,
    Head,
    Options,
    Trace,
    Connect,
    /// Any HTTP method is allowed
    Any,
}

impl From<String> for HttpMethod {
    fn from(s: String) -> Self {
        match s.to_lowercase().as_str() {
            "get" => HttpMethod::Get,
            "post" => HttpMethod::Post,
            "put" => HttpMethod::Put,
            "delete" => HttpMethod::Delete,
            "patch" => HttpMethod::Patch,
            "head" => HttpMethod::Head,
            "options" => HttpMethod::Options,
            "trace" => HttpMethod::Trace,
            "connect" => HttpMethod::Connect,
            "*" => HttpMethod::Any,
            _ => panic!("Invalid HTTP method"),
        }
    }
}

impl From<&str> for HttpMethod {
    fn from(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "get" => HttpMethod::Get,
            "post" => HttpMethod::Post,
            "put" => HttpMethod::Put,
            "delete" => HttpMethod::Delete,
            "patch" => HttpMethod::Patch,
            "head" => HttpMethod::Head,
            "options" => HttpMethod::Options,
            "trace" => HttpMethod::Trace,
            "connect" => HttpMethod::Connect,
            "*" => HttpMethod::Any,
            _ => panic!("Invalid HTTP method"),
        }
    }
}

impl std::fmt::Display for HttpMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HttpMethod::Get => write!(f, "GET"),
            HttpMethod::Post => write!(f, "POST"),
            HttpMethod::Put => write!(f, "PUT"),
            HttpMethod::Delete => write!(f, "DELETE"),
            HttpMethod::Patch => write!(f, "PATCH"),
            HttpMethod::Head => write!(f, "HEAD"),
            HttpMethod::Options => write!(f, "OPTIONS"),
            HttpMethod::Trace => write!(f, "TRACE"),
            HttpMethod::Connect => write!(f, "CONNECT"),
            HttpMethod::Any => write!(f, "*"),
        }
    }
}
