use trustfall::provider::{
    AsVertex, ContextIterator, ContextOutcomeIterator, EdgeParameters, ResolveEdgeInfo,
    VertexIterator,
};

use super::vertex::Vertex;

pub(super) fn resolve_operation_edge<'a, V: AsVertex<Vertex> + 'a>(
    contexts: ContextIterator<'a, V>,
    edge_name: &str,
    _parameters: &EdgeParameters,
    resolve_info: &ResolveEdgeInfo,
) -> ContextOutcomeIterator<'a, V, VertexIterator<'a, Vertex>> {
    match edge_name {
        "xAmazonApigatewayIntegration" => {
            operation::x_amazon_apigateway_integration(contexts, resolve_info)
        }
        _ => {
            unreachable!("attempted to resolve unexpected edge '{edge_name}' on type 'Operation'")
        }
    }
}

mod operation {

    use trustfall::provider::{
        resolve_neighbors_with, AsVertex, ContextIterator, ContextOutcomeIterator, ResolveEdgeInfo,
        VertexIterator,
    };

    use crate::utils::Operator;

    use super::super::vertex::Vertex;

    pub(super) fn x_amazon_apigateway_integration<'a, V: AsVertex<Vertex> + 'a>(
        contexts: ContextIterator<'a, V>,
        _resolve_info: &ResolveEdgeInfo,
    ) -> ContextOutcomeIterator<'a, V, VertexIterator<'a, Vertex>> {
        resolve_neighbors_with(contexts, move |vertex| {
            let operation: &Operator = vertex
                .as_operation()
                .expect("conversion failed, vertex was not a Operation");

            match &operation.aws {
                Some(value) => Box::new(std::iter::once(Vertex::AmazonApigatewayIntegration(
                    value.clone(),
                ))),
                None => Box::new(std::iter::empty()),
            }
        })
    }
}

pub(super) fn resolve_path_edge<'a, V: AsVertex<Vertex> + 'a>(
    contexts: ContextIterator<'a, V>,
    edge_name: &str,
    _parameters: &EdgeParameters,
    resolve_info: &ResolveEdgeInfo,
) -> ContextOutcomeIterator<'a, V, VertexIterator<'a, Vertex>> {
    match edge_name {
        "delete" => path::delete(contexts, resolve_info),
        "get" => path::get(contexts, resolve_info),
        "operations" => path::operations(contexts, resolve_info),
        "options" => path::options(contexts, resolve_info),
        "patch" => path::patch(contexts, resolve_info),
        "post" => path::post(contexts, resolve_info),
        "put" => path::put(contexts, resolve_info),
        _ => {
            unreachable!("attempted to resolve unexpected edge '{edge_name}' on type 'Path'")
        }
    }
}

mod path {

    use trustfall::provider::{
        resolve_neighbors_with, AsVertex, ContextIterator, ContextOutcomeIterator, ResolveEdgeInfo,
        VertexIterator,
    };

    use crate::utils::Route;

    use super::super::vertex::Vertex;

    pub(super) fn delete<'a, V: AsVertex<Vertex> + 'a>(
        contexts: ContextIterator<'a, V>,
        _resolve_info: &ResolveEdgeInfo,
    ) -> ContextOutcomeIterator<'a, V, VertexIterator<'a, Vertex>> {
        resolve_neighbors_with(contexts, move |vertex| {
            let route: &Route = vertex
                .as_path()
                .expect("conversion failed, vertex was not a Path");

            match &route.delete {
                Some(op) => Box::new(std::iter::once(Vertex::Operation(op.clone()))),
                None => Box::new(std::iter::empty()),
            }
        })
    }

    pub(super) fn get<'a, V: AsVertex<Vertex> + 'a>(
        contexts: ContextIterator<'a, V>,
        _resolve_info: &ResolveEdgeInfo,
    ) -> ContextOutcomeIterator<'a, V, VertexIterator<'a, Vertex>> {
        resolve_neighbors_with(contexts, move |vertex| {
            let route: &Route = vertex
                .as_path()
                .expect("conversion failed, vertex was not a Path");

            match &route.get {
                Some(op) => Box::new(std::iter::once(Vertex::Operation(op.clone()))),
                None => Box::new(std::iter::empty()),
            }
        })
    }

    pub(super) fn operations<'a, V: AsVertex<Vertex> + 'a>(
        contexts: ContextIterator<'a, V>,
        _resolve_info: &ResolveEdgeInfo,
    ) -> ContextOutcomeIterator<'a, V, VertexIterator<'a, Vertex>> {
        resolve_neighbors_with(contexts, move |vertex| {
            let route: &Route = vertex
                .as_path()
                .expect("conversion failed, vertex was not a Path");

            let mut operations = Vec::new();
            if let Some(op) = &route.delete {
                operations.push(Vertex::Operation(op.clone()));
            }
            if let Some(op) = &route.get {
                operations.push(Vertex::Operation(op.clone()));
            }
            if let Some(op) = &route.options {
                operations.push(Vertex::Operation(op.clone()));
            }
            if let Some(op) = &route.patch {
                operations.push(Vertex::Operation(op.clone()));
            }
            if let Some(op) = &route.post {
                operations.push(Vertex::Operation(op.clone()));
            }
            if let Some(op) = &route.put {
                operations.push(Vertex::Operation(op.clone()));
            }
            Box::new(operations.into_iter())
        })
    }

    pub(super) fn options<'a, V: AsVertex<Vertex> + 'a>(
        contexts: ContextIterator<'a, V>,
        _resolve_info: &ResolveEdgeInfo,
    ) -> ContextOutcomeIterator<'a, V, VertexIterator<'a, Vertex>> {
        resolve_neighbors_with(contexts, move |vertex| {
            let route: &Route = vertex
                .as_path()
                .expect("conversion failed, vertex was not a Path");

            match &route.options {
                Some(op) => Box::new(std::iter::once(Vertex::Operation(op.clone()))),
                None => Box::new(std::iter::empty()),
            }
        })
    }

    pub(super) fn patch<'a, V: AsVertex<Vertex> + 'a>(
        contexts: ContextIterator<'a, V>,
        _resolve_info: &ResolveEdgeInfo,
    ) -> ContextOutcomeIterator<'a, V, VertexIterator<'a, Vertex>> {
        resolve_neighbors_with(contexts, move |vertex| {
            let route: &Route = vertex
                .as_path()
                .expect("conversion failed, vertex was not a Path");

            match &route.patch {
                Some(op) => Box::new(std::iter::once(Vertex::Operation(op.clone()))),
                None => Box::new(std::iter::empty()),
            }
        })
    }

    pub(super) fn post<'a, V: AsVertex<Vertex> + 'a>(
        contexts: ContextIterator<'a, V>,
        _resolve_info: &ResolveEdgeInfo,
    ) -> ContextOutcomeIterator<'a, V, VertexIterator<'a, Vertex>> {
        resolve_neighbors_with(contexts, move |vertex| {
            let route: &Route = vertex
                .as_path()
                .expect("conversion failed, vertex was not a Path");

            match &route.post {
                Some(op) => Box::new(std::iter::once(Vertex::Operation(op.clone()))),
                None => Box::new(std::iter::empty()),
            }
        })
    }

    pub(super) fn put<'a, V: AsVertex<Vertex> + 'a>(
        contexts: ContextIterator<'a, V>,
        _resolve_info: &ResolveEdgeInfo,
    ) -> ContextOutcomeIterator<'a, V, VertexIterator<'a, Vertex>> {
        resolve_neighbors_with(contexts, move |vertex| {
            let route: &Route = vertex
                .as_path()
                .expect("conversion failed, vertex was not a Path");

            match &route.put {
                Some(op) => Box::new(std::iter::once(Vertex::Operation(op.clone()))),
                None => Box::new(std::iter::empty()),
            }
        })
    }
}
