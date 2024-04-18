use std::{
    path::PathBuf,
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

use crate::{errors::OpenAPIAdapterErrors, utils::find_files};

use super::{
    utils::{merge, Route},
    vertex::Vertex,
};

static SCHEMA: OnceLock<Schema> = OnceLock::new();

#[non_exhaustive]
#[derive(Debug, Default)]
pub struct OpenApiAdapter {
    openapi: openapiv3::OpenAPI,
}

impl OpenApiAdapter {
    pub const SCHEMA_TEXT: &'static str = include_str!("./schema.graphql");

    pub fn schema() -> &'static Schema {
        SCHEMA.get_or_init(|| Schema::parse(Self::SCHEMA_TEXT).expect("not a valid schema"))
    }

    /// Create a new OpenAPI default adapter
    pub fn new() -> Self {
        Default::default()
    }

    /// New instance with selected files to used
    pub fn new_with_files(files: Vec<PathBuf>) -> Result<Self, OpenAPIAdapterErrors> {
        let mut adapter = Self::default();
        adapter.files(files)?;
        Ok(adapter)
    }

    pub fn new_with_path(path: PathBuf) -> Result<Self, OpenAPIAdapterErrors> {
        let mut adapter = Self::default();
        adapter.set_path(path)?;
        Ok(adapter)
    }

    /// Set the files that are to be used
    pub fn files(&mut self, files: Vec<PathBuf>) -> Result<(), OpenAPIAdapterErrors> {
        let merged_content = merge(files)?;
        self.openapi = serde_yaml::from_str(&merged_content)
            .map_err(OpenAPIAdapterErrors::FailedToSerializeToOpenAPI)?;
        Ok(())
    }

    /// Sets the directory that files are to be found
    pub fn set_path(&mut self, path: PathBuf) -> Result<(), OpenAPIAdapterErrors> {
        if !path.exists() {
            return Err(OpenAPIAdapterErrors::PathIsNotADirectory(path));
        }
        let mut files = find_files(&path, "yaml".as_ref());
        files.extend(find_files(&path, "yml".as_ref()));
        if files.is_empty() {
            return Err(OpenAPIAdapterErrors::FilesNotFound(path));
        }
        let merged_content = merge(files)?;
        self.openapi = serde_yaml::from_str(&merged_content)
            .map_err(OpenAPIAdapterErrors::FailedToSerializeToOpenAPI)?;
        Ok(())
    }

    fn info(&self) -> Vertex {
        Vertex::Info(self.openapi.info.clone())
    }

    fn path(&self, path: &str) -> Vertex {
        let mut route: Route = self
            .openapi
            .paths
            .paths
            .get(path)
            .expect("path not found")
            .clone()
            .into();
        route.path = path.to_string();
        Vertex::Path(route)
    }

    fn paths<'a>(&self) -> VertexIterator<'a, Vertex> {
        let iter = self.openapi.paths.clone().into_iter().map(|x| {
            let mut route: Route = x.1.into();
            route.path = x.0.clone().to_string();
            Vertex::Path(route)
        });
        Box::new(iter)
    }

    fn tags<'a>(&self) -> VertexIterator<'a, Vertex> {
        let iter = self.openapi.tags.clone().into_iter().map(Vertex::Tag);
        Box::new(iter)
        // Vertex::Tags(self.openapi.tags.clone())
    }
}

impl<'a> trustfall::provider::Adapter<'a> for OpenApiAdapter {
    type Vertex = Vertex;

    fn resolve_starting_vertices(
        &self,
        edge_name: &Arc<str>,
        parameters: &EdgeParameters,
        _resolve_info: &ResolveInfo,
    ) -> VertexIterator<'a, Self::Vertex> {
        match edge_name.as_ref() {
            "Info" => Box::new(std::iter::once(self.info())),
            "Path" => {
                let path: &str = parameters
                    .get("path")
                    .expect(
                        "failed to find parameter 'path' when resolving 'Path' starting vertices",
                    )
                    .as_str()
                    .expect(
                        "unexpected null or other incorrect data type for Trustfall type 'String!'",
                    );
                // super::entrypoints::path(path, resolve_info, &self.openapi)
                Box::new(std::iter::once(self.path(path)))
            }
            // "Paths" => super::entrypoints::paths(resolve_info, &self.openapi),
            "Paths" => self.paths(),
            // "Tags" => super::entrypoints::tags(resolve_info, &self.openapi),
            // "Tags" => Box::new(std::iter::once(self.tags())),
            "Tags" => self.tags(),
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
            "AmazonApigatewayIntegration" => {
                super::properties::resolve_amazon_apigateway_integration_property(
                    contexts,
                    property_name.as_ref(),
                    resolve_info,
                )
            }
            "Info" => super::properties::resolve_info_property(
                contexts,
                property_name.as_ref(),
                resolve_info,
            ),
            "Operation" => super::properties::resolve_operation_property(
                contexts,
                property_name.as_ref(),
                resolve_info,
            ),
            "Path" => super::properties::resolve_path_property(
                contexts,
                property_name.as_ref(),
                resolve_info,
            ),
            "Tag" => super::properties::resolve_tag_property(
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
            "Operation" => super::edges::resolve_operation_edge(
                contexts,
                edge_name.as_ref(),
                parameters,
                resolve_info,
            ),
            "Path" => super::edges::resolve_path_edge(
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
