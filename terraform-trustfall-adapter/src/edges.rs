use trustfall::provider::{
    AsVertex, ContextIterator, ContextOutcomeIterator, EdgeParameters, ResolveEdgeInfo,
    VertexIterator,
};

use super::vertex::Vertex;

pub(super) fn resolve_api_config_edge<'a, V: AsVertex<Vertex> + 'a>(
    contexts: ContextIterator<'a, V>,
    edge_name: &str,
    _parameters: &EdgeParameters,
    resolve_info: &ResolveEdgeInfo,
) -> ContextOutcomeIterator<'a, V, VertexIterator<'a, Vertex>> {
    match edge_name {
        "template_variables" => api_config::template_variables(contexts, resolve_info),
        _ => {
            unreachable!("attempted to resolve unexpected edge '{edge_name}' on type 'ApiConfig'")
        }
    }
}

mod api_config {
    use trustfall::provider::{
        resolve_neighbors_with, AsVertex, ContextIterator, ContextOutcomeIterator, ResolveEdgeInfo,
        VertexIterator,
    };

    use crate::model::ApiConfig;

    use super::super::vertex::Vertex;

    pub(super) fn template_variables<'a, V: AsVertex<Vertex> + 'a>(
        contexts: ContextIterator<'a, V>,
        _resolve_info: &ResolveEdgeInfo,
    ) -> ContextOutcomeIterator<'a, V, VertexIterator<'a, Vertex>> {
        resolve_neighbors_with(contexts, move |vertex| {
            let vertex: &ApiConfig = vertex
                .as_api_config()
                .expect("conversion failed, vertex was not a ApiConfig");
            Box::new(
                vertex
                    .template_variables
                    .clone()
                    .into_iter()
                    .map(Vertex::TemplateVariable),
            )
        })
    }
}

pub(super) fn resolve_lambda_edge<'a, V: AsVertex<Vertex> + 'a>(
    contexts: ContextIterator<'a, V>,
    edge_name: &str,
    _parameters: &EdgeParameters,
    resolve_info: &ResolveEdgeInfo,
) -> ContextOutcomeIterator<'a, V, VertexIterator<'a, Vertex>> {
    match edge_name {
        "permissions" => lambda::permissions(contexts, resolve_info),
        _ => {
            unreachable!("attempted to resolve unexpected edge '{edge_name}' on type 'Lambda'")
        }
    }
}

mod lambda {
    use trustfall::provider::{
        resolve_neighbors_with, AsVertex, ContextIterator, ContextOutcomeIterator, ResolveEdgeInfo,
        VertexIterator,
    };

    use crate::model::Lambda;

    use super::super::vertex::Vertex;

    pub(super) fn permissions<'a, V: AsVertex<Vertex> + 'a>(
        contexts: ContextIterator<'a, V>,
        _resolve_info: &ResolveEdgeInfo,
    ) -> ContextOutcomeIterator<'a, V, VertexIterator<'a, Vertex>> {
        resolve_neighbors_with(contexts, move |vertex| {
            let vertex: &Lambda = vertex
                .as_lambda()
                .expect("conversion failed, vertex was not a Lambda");
            Box::new(
                vertex
                    .permissions
                    .clone()
                    .into_iter()
                    .map(Vertex::Permissions),
            )
        })
    }
}

pub(super) fn resolve_module_edge<'a, V: AsVertex<Vertex> + 'a>(
    contexts: ContextIterator<'a, V>,
    edge_name: &str,
    _parameters: &EdgeParameters,
    resolve_info: &ResolveEdgeInfo,
) -> ContextOutcomeIterator<'a, V, VertexIterator<'a, Vertex>> {
    match edge_name {
        "variables" => module::variables(contexts, resolve_info),
        _ => {
            unreachable!("attempted to resolve unexpected edge '{edge_name}' on type 'Module'")
        }
    }
}

mod module {
    use trustfall::provider::{
        resolve_neighbors_with, AsVertex, ContextIterator, ContextOutcomeIterator, ResolveEdgeInfo,
        VertexIterator,
    };

    use crate::model::Module;

    use super::super::vertex::Vertex;

    pub(super) fn variables<'a, V: AsVertex<Vertex> + 'a>(
        contexts: ContextIterator<'a, V>,
        _resolve_info: &ResolveEdgeInfo,
    ) -> ContextOutcomeIterator<'a, V, VertexIterator<'a, Vertex>> {
        resolve_neighbors_with(contexts, move |vertex| {
            let vertex: &Module = vertex
                .as_module()
                .expect("conversion failed, vertex was not a Module");
            Box::new(vertex.variables.clone().into_iter().map(Vertex::Variable))
        })
    }
}

pub(super) fn resolve_template_variable_edge<'a, V: AsVertex<Vertex> + 'a>(
    contexts: ContextIterator<'a, V>,
    edge_name: &str,
    _parameters: &EdgeParameters,
    resolve_info: &ResolveEdgeInfo,
) -> ContextOutcomeIterator<'a, V, VertexIterator<'a, Vertex>> {
    match edge_name {
        "lambda" => template_variable::lambda(contexts, resolve_info),
        _ => {
            unreachable!(
                "attempted to resolve unexpected edge '{edge_name}' on type 'TemplateVariable'"
            )
        }
    }
}

mod template_variable {
    use trustfall::provider::{
        resolve_neighbors_with, AsVertex, ContextIterator, ContextOutcomeIterator, ResolveEdgeInfo,
        VertexIterator,
    };

    use crate::model::TemplateVariable;

    use super::super::vertex::Vertex;

    pub(super) fn lambda<'a, V: AsVertex<Vertex> + 'a>(
        contexts: ContextIterator<'a, V>,
        _resolve_info: &ResolveEdgeInfo,
    ) -> ContextOutcomeIterator<'a, V, VertexIterator<'a, Vertex>> {
        resolve_neighbors_with(contexts, move |vertex| {
            let vertex: &TemplateVariable = vertex
                .as_template_variable()
                .expect("conversion failed, vertex was not a TemplateVariable");
            match &vertex.lambda {
                Some(s) => Box::new(std::iter::once(Vertex::Lambda(s.clone()))),
                None => Box::new(std::iter::empty()),
            }
        })
    }
}

pub(super) fn resolve_terraform_edge<'a, V: AsVertex<Vertex> + 'a>(
    contexts: ContextIterator<'a, V>,
    edge_name: &str,
    _parameters: &EdgeParameters,
    resolve_info: &ResolveEdgeInfo,
) -> ContextOutcomeIterator<'a, V, VertexIterator<'a, Vertex>> {
    match edge_name {
        "backend" => terraform::backend(contexts, resolve_info),
        "required_providers" => terraform::required_providers(contexts, resolve_info),
        _ => {
            unreachable!("attempted to resolve unexpected edge '{edge_name}' on type 'Terraform'")
        }
    }
}

mod terraform {
    use trustfall::provider::{
        resolve_neighbors_with, AsVertex, ContextIterator, ContextOutcomeIterator, ResolveEdgeInfo,
        VertexIterator,
    };

    use crate::model::Terraform;

    use super::super::vertex::Vertex;

    pub(super) fn backend<'a, V: AsVertex<Vertex> + 'a>(
        contexts: ContextIterator<'a, V>,
        _resolve_info: &ResolveEdgeInfo,
    ) -> ContextOutcomeIterator<'a, V, VertexIterator<'a, Vertex>> {
        resolve_neighbors_with(contexts, move |vertex| {
            let vertex: &Terraform = vertex
                .as_terraform()
                .expect("conversion failed, vertex was not a Terraform");
            match &vertex.backend {
                Some(s) => Box::new(std::iter::once(Vertex::Backend(s.clone()))),
                None => Box::new(std::iter::empty()),
            }
        })
    }

    pub(super) fn required_providers<'a, V: AsVertex<Vertex> + 'a>(
        contexts: ContextIterator<'a, V>,
        _resolve_info: &ResolveEdgeInfo,
    ) -> ContextOutcomeIterator<'a, V, VertexIterator<'a, Vertex>> {
        resolve_neighbors_with(contexts, move |vertex| {
            let vertex: &Terraform = vertex
                .as_terraform()
                .expect("conversion failed, vertex was not a Terraform");
            Box::new(
                vertex
                    .required_providers
                    .clone()
                    .into_iter()
                    .map(Vertex::RequiredProvider),
            )
        })
    }
}
