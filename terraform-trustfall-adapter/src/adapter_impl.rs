use std::{
    path::{Path, PathBuf},
    sync::{Arc, OnceLock},
};

use trustfall::{
    provider::{
        resolve_coercion_using_schema, resolve_property_with, AsVertex, ContextIterator,
        ContextOutcomeIterator, EdgeParameters, ResolveEdgeInfo, ResolveInfo, Typename,
        VertexIterator,
    },
    FieldValue, Schema,
};

use crate::utils::extract_data_from_files;

use super::{
    model::{Module, HCL},
    utils::extract_data_from_hcl,
    vertex::Vertex,
};

static SCHEMA: OnceLock<Schema> = OnceLock::new();

#[non_exhaustive]
#[derive(Debug, Default)]
pub struct HclAdapter {
    data: HCL,
}

impl HclAdapter {
    pub const SCHEMA_TEXT: &'static str = include_str!("./schema.graphql");

    pub fn schema() -> &'static Schema {
        SCHEMA.get_or_init(|| Schema::parse(Self::SCHEMA_TEXT).expect("not a valid schema"))
    }

    /// Create a new HCL adapter from a folder containing HCL files. It will ignore the .terraform folder
    pub fn new(path: &Path) -> Self {
        Self {
            data: extract_data_from_hcl(path),
        }
    }

    pub fn new_with_files(files: Vec<PathBuf>) -> Self {
        Self {
            data: extract_data_from_files(&files),
        }
    }
}

impl<'a> trustfall::provider::Adapter<'a> for HclAdapter {
    type Vertex = Vertex;

    fn resolve_starting_vertices(
        &self,
        edge_name: &Arc<str>,
        parameters: &EdgeParameters,
        _resolve_info: &ResolveInfo,
    ) -> VertexIterator<'a, Self::Vertex> {
        match edge_name.as_ref() {
            "ApiConfig" => match &self.data.api_config {
                Some(config) => Box::new(std::iter::once(Vertex::ApiConfig(config.clone()))),
                None => Box::new(std::iter::empty()),
            },
            "Lambda" => {
                let iter = self.data.lambda.clone().into_iter().map(Vertex::Lambda);
                Box::new(iter)
            }
            "Module" => {
                let name: &str = parameters
                    .get("name")
                    .expect(
                        "failed to find parameter 'name' when resolving 'Module' starting vertices",
                    )
                    .as_str()
                    .expect(
                        "unexpected null or other incorrect data type for Trustfall type 'String!'",
                    );
                let tag: Option<&str> = parameters
                    .get("tag")
                    .expect(
                        "failed to find parameter 'tag' when resolving 'Module' starting vertices",
                    )
                    .as_str();
                let module_tag = tag;
                let iter: Vec<Vertex> = self
                    .data
                    .raw
                    .clone()
                    .into_iter()
                    .filter_map(move |value| {
                        if let Some(tag) = module_tag {
                            value
                                .get(name)
                                .and_then(|x| x.get(tag))
                                .map(|v| Vertex::Module(Module::from_serde(v.clone(), tag)))
                        } else {
                            value
                                .get(name)
                                .map(|v| Vertex::Module(Module::from_serde(v.clone(), name)))
                        }
                    })
                    .collect();
                Box::new(iter.into_iter())
            }
            "Modules" => {
                let iter = self.data.modules.clone().into_iter().map(Vertex::Module);
                Box::new(iter)
            }
            _ => {
                unreachable!(
                    "attempted to resolve starting vertices for unexpected edge name: {edge_name}"
                )
            }
        }
    }

    fn resolve_property<V: AsVertex<Self::Vertex> + 'a>(
        &self,
        contexts: ContextIterator<'a, V>,
        type_name: &Arc<str>,
        property_name: &Arc<str>,
        resolve_info: &ResolveInfo,
    ) -> ContextOutcomeIterator<'a, V, FieldValue> {
        if property_name.as_ref() == "__typename" {
            return resolve_property_with(contexts, |vertex| vertex.typename().into());
        }
        match type_name.as_ref() {
            "ApiConfig" => super::properties::resolve_api_config_property(
                contexts,
                property_name.as_ref(),
                resolve_info,
            ),
            "Backend" => super::properties::resolve_backend_property(
                contexts,
                property_name.as_ref(),
                resolve_info,
            ),
            "Lambda" => super::properties::resolve_lambda_property(
                contexts,
                property_name.as_ref(),
                resolve_info,
            ),
            "Module" => super::properties::resolve_module_property(
                contexts,
                property_name.as_ref(),
                resolve_info,
            ),
            "Permissions" => super::properties::resolve_permissions_property(
                contexts,
                property_name.as_ref(),
                resolve_info,
            ),
            "RequiredProvider" => super::properties::resolve_required_provider_property(
                contexts,
                property_name.as_ref(),
                resolve_info,
            ),
            "TemplateVariable" => super::properties::resolve_template_variable_property(
                contexts,
                property_name.as_ref(),
                resolve_info,
            ),
            "Terraform" => super::properties::resolve_terraform_property(
                contexts,
                property_name.as_ref(),
                resolve_info,
            ),
            "Variable" => super::properties::resolve_variable_property(
                contexts,
                property_name.as_ref(),
                resolve_info,
            ),
            _ => {
                unreachable!(
                    "attempted to read property '{property_name}' on unexpected type: {type_name}"
                )
            }
        }
    }

    fn resolve_neighbors<V: AsVertex<Self::Vertex> + 'a>(
        &self,
        contexts: ContextIterator<'a, V>,
        type_name: &Arc<str>,
        edge_name: &Arc<str>,
        parameters: &EdgeParameters,
        resolve_info: &ResolveEdgeInfo,
    ) -> ContextOutcomeIterator<'a, V, VertexIterator<'a, Self::Vertex>> {
        match type_name.as_ref() {
            "ApiConfig" => super::edges::resolve_api_config_edge(
                contexts,
                edge_name.as_ref(),
                parameters,
                resolve_info,
            ),
            "Lambda" => super::edges::resolve_lambda_edge(
                contexts,
                edge_name.as_ref(),
                parameters,
                resolve_info,
            ),
            "Module" => super::edges::resolve_module_edge(
                contexts,
                edge_name.as_ref(),
                parameters,
                resolve_info,
            ),
            "TemplateVariable" => super::edges::resolve_template_variable_edge(
                contexts,
                edge_name.as_ref(),
                parameters,
                resolve_info,
            ),
            "Terraform" => super::edges::resolve_terraform_edge(
                contexts,
                edge_name.as_ref(),
                parameters,
                resolve_info,
            ),
            _ => {
                unreachable!(
                    "attempted to resolve edge '{edge_name}' on unexpected type: {type_name}"
                )
            }
        }
    }

    fn resolve_coercion<V: AsVertex<Self::Vertex> + 'a>(
        &self,
        contexts: ContextIterator<'a, V>,
        _type_name: &Arc<str>,
        coerce_to_type: &Arc<str>,
        _resolve_info: &ResolveInfo,
    ) -> ContextOutcomeIterator<'a, V, bool> {
        resolve_coercion_using_schema(contexts, Self::schema(), coerce_to_type.as_ref())
    }
}
