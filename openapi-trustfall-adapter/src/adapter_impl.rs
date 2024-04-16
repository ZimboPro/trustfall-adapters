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

use crate::utils::{find_files, open_file};

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

    /// Create a new OpenAPI adapter from a file or directory
    ///
    /// If a directory, yaml and yml files in it and merges them into a single OpenAPI file
    /// If a file, reads the file and parses it
    pub fn new(path: PathBuf) -> Self {
        let openapi = if path.is_dir() {
            let mut files = find_files(&path, "yaml".as_ref());
            files.extend(find_files(&path, "yml".as_ref()));
            let mut files_content = Vec::new();
            for file in files {
                files_content.push(open_file(file));
            }
            let merged_content = merge(files_content);
            serde_yaml::from_str(&merged_content).unwrap()
        } else if path.is_file() {
            let file = std::fs::File::open(path).expect("failed to open file");
            let reader = std::io::BufReader::new(file);
            serde_yaml::from_reader(reader).expect("failed to parse OpenAPI file")
        } else {
            panic!("Path: {:?} is not a file or directory", path)
        };

        Self { openapi }
    }

    pub fn new_files(files: Vec<PathBuf>) -> Self {
        let mut files_content = Vec::new();
        for file in files {
            files_content.push(open_file(file));
        }
        let merged_content = merge(files_content);
        let openapi: openapiv3::OpenAPI = serde_yaml::from_str(&merged_content).unwrap();
        Self { openapi }
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
