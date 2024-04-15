use trustfall::{
    provider::{
        field_property, resolve_property_with, AsVertex, ContextIterator, ContextOutcomeIterator,
        ResolveInfo,
    },
    FieldValue,
};

use super::vertex::Vertex;

pub(super) fn resolve_amazon_apigateway_integration_property<'a, V: AsVertex<Vertex> + 'a>(
    contexts: ContextIterator<'a, V>,
    property_name: &str,
    _resolve_info: &ResolveInfo,
) -> ContextOutcomeIterator<'a, V, FieldValue> {
    match property_name {
        "arn" => resolve_property_with(
            contexts,
            field_property!(as_amazon_apigateway_integration, arn),
        ),
        "httpMethod" => resolve_property_with(
            contexts,
            field_property!(as_amazon_apigateway_integration, http_method),
        ),
        "passthroughBehavior" => resolve_property_with(
            contexts,
            field_property!(as_amazon_apigateway_integration, pass_through_behavior),
        ),
        "timeoutInMillis" => resolve_property_with(contexts, |vertex: &Vertex| {
            let t = vertex.as_amazon_apigateway_integration().unwrap();
            if let Some(timeout) = t.timeout_in_millis {
                FieldValue::Uint64(timeout as u64)
            } else {
                FieldValue::Null
            }
        }),
        "trigger" => resolve_property_with(
            contexts,
            field_property!(as_amazon_apigateway_integration, trigger),
        ),
        "type" => resolve_property_with(
            contexts,
            field_property!(as_amazon_apigateway_integration, r_type),
        ),
        "uri" => resolve_property_with(
            contexts,
            field_property!(as_amazon_apigateway_integration, uri),
        ),
        _ => {
            unreachable!(
                "attempted to read unexpected property '{property_name}' on type 'AmazonApigatewayIntegration'"
            )
        }
    }
}

pub(super) fn resolve_info_property<'a, V: AsVertex<Vertex> + 'a>(
    contexts: ContextIterator<'a, V>,
    property_name: &str,
    _resolve_info: &ResolveInfo,
) -> ContextOutcomeIterator<'a, V, FieldValue> {
    match property_name {
        "description" => resolve_property_with(contexts, field_property!(as_info, description)),
        "title" => resolve_property_with(contexts, field_property!(as_info, title)),
        "version" => resolve_property_with(contexts, field_property!(as_info, version)),
        _ => {
            unreachable!("attempted to read unexpected property '{property_name}' on type 'Info'")
        }
    }
}

pub(super) fn resolve_operation_property<'a, V: AsVertex<Vertex> + 'a>(
    contexts: ContextIterator<'a, V>,
    property_name: &str,
    _resolve_info: &ResolveInfo,
) -> ContextOutcomeIterator<'a, V, FieldValue> {
    match property_name {
        "description" => {
            resolve_property_with(contexts, field_property!(as_operation, description))
        }
        "summary" => resolve_property_with(contexts, field_property!(as_operation, summary)),
        "method" => resolve_property_with(contexts, field_property!(as_operation, method)),
        "tags" => resolve_property_with(contexts, field_property!(as_operation, tags)),
        _ => {
            unreachable!(
                "attempted to read unexpected property '{property_name}' on type 'Operation'"
            )
        }
    }
}

pub(super) fn resolve_path_property<'a, V: AsVertex<Vertex> + 'a>(
    contexts: ContextIterator<'a, V>,
    property_name: &str,
    _resolve_info: &ResolveInfo,
) -> ContextOutcomeIterator<'a, V, FieldValue> {
    match property_name {
        "path" => resolve_property_with(contexts, field_property!(as_path, path)),
        _ => {
            unreachable!("attempted to read unexpected property '{property_name}' on type 'Path'")
        }
    }
}

pub(super) fn resolve_tag_property<'a, V: AsVertex<Vertex> + 'a>(
    contexts: ContextIterator<'a, V>,
    property_name: &str,
    _resolve_info: &ResolveInfo,
) -> ContextOutcomeIterator<'a, V, FieldValue> {
    match property_name {
        "description" => resolve_property_with(contexts, field_property!(as_tag, description)),
        "name" => resolve_property_with(contexts, field_property!(as_tag, name)),
        _ => {
            unreachable!("attempted to read unexpected property '{property_name}' on type 'Tag'")
        }
    }
}
