use trustfall::{
    provider::{
        field_property, resolve_property_with, AsVertex, ContextIterator, ContextOutcomeIterator,
        ResolveInfo,
    },
    FieldValue,
};

use super::vertex::Vertex;

pub(super) fn resolve_api_config_property<'a, V: AsVertex<Vertex> + 'a>(
    contexts: ContextIterator<'a, V>,
    property_name: &str,
    _resolve_info: &ResolveInfo,
) -> ContextOutcomeIterator<'a, V, FieldValue> {
    match property_name {
        "source" => resolve_property_with(contexts, field_property!(as_api_config, source)),
        "template_file" => {
            resolve_property_with(contexts, field_property!(as_api_config, template_file))
        }
        "version" => resolve_property_with(contexts, field_property!(as_api_config, version)),
        _ => {
            unreachable!(
                "attempted to read unexpected property '{property_name}' on type 'ApiConfig'"
            )
        }
    }
}

pub(super) fn resolve_backend_property<'a, V: AsVertex<Vertex> + 'a>(
    contexts: ContextIterator<'a, V>,
    property_name: &str,
    _resolve_info: &ResolveInfo,
) -> ContextOutcomeIterator<'a, V, FieldValue> {
    match property_name {
        "name" => resolve_property_with(contexts, field_property!(as_backend, name)),
        _ => {
            unreachable!(
                "attempted to read unexpected property '{property_name}' on type 'Backend'"
            )
        }
    }
}

pub(super) fn resolve_lambda_property<'a, V: AsVertex<Vertex> + 'a>(
    contexts: ContextIterator<'a, V>,
    property_name: &str,
    _resolve_info: &ResolveInfo,
) -> ContextOutcomeIterator<'a, V, FieldValue> {
    match property_name {
        "description" => resolve_property_with(contexts, field_property!(as_lambda, description)),
        "handler" => resolve_property_with(contexts, field_property!(as_lambda, handler)),
        "name" => resolve_property_with(contexts, field_property!(as_lambda, name)),
        _ => {
            unreachable!("attempted to read unexpected property '{property_name}' on type 'Lambda'")
        }
    }
}

pub(super) fn resolve_module_property<'a, V: AsVertex<Vertex> + 'a>(
    contexts: ContextIterator<'a, V>,
    property_name: &str,
    _resolve_info: &ResolveInfo,
) -> ContextOutcomeIterator<'a, V, FieldValue> {
    match property_name {
        "source" => resolve_property_with(contexts, field_property!(as_module, source)),
        "version" => resolve_property_with(contexts, field_property!(as_module, version)),
        _ => {
            unreachable!("attempted to read unexpected property '{property_name}' on type 'Module'")
        }
    }
}

pub(super) fn resolve_permissions_property<'a, V: AsVertex<Vertex> + 'a>(
    contexts: ContextIterator<'a, V>,
    property_name: &str,
    _resolve_info: &ResolveInfo,
) -> ContextOutcomeIterator<'a, V, FieldValue> {
    match property_name {
        "http_method" => {
            resolve_property_with(contexts, field_property!(as_permissions, http_method))
        }
        "http_path" => resolve_property_with(contexts, field_property!(as_permissions, http_path)),
        "principal" => resolve_property_with(contexts, field_property!(as_permissions, principal)),
        "source_arn" => {
            resolve_property_with(contexts, field_property!(as_permissions, source_arn))
        }
        "statement_id" => {
            resolve_property_with(contexts, field_property!(as_permissions, statement_id))
        }
        _ => {
            unreachable!(
                "attempted to read unexpected property '{property_name}' on type 'Permissions'"
            )
        }
    }
}

pub(super) fn resolve_required_provider_property<'a, V: AsVertex<Vertex> + 'a>(
    contexts: ContextIterator<'a, V>,
    property_name: &str,
    _resolve_info: &ResolveInfo,
) -> ContextOutcomeIterator<'a, V, FieldValue> {
    match property_name {
        "name" => resolve_property_with(contexts, field_property!(as_required_provider, name)),
        "source" => resolve_property_with(contexts, field_property!(as_required_provider, source)),
        "version" => {
            resolve_property_with(contexts, field_property!(as_required_provider, version))
        }
        _ => {
            unreachable!(
                "attempted to read unexpected property '{property_name}' on type 'RequiredProvider'"
            )
        }
    }
}

pub(super) fn resolve_template_variable_property<'a, V: AsVertex<Vertex> + 'a>(
    contexts: ContextIterator<'a, V>,
    property_name: &str,
    _resolve_info: &ResolveInfo,
) -> ContextOutcomeIterator<'a, V, FieldValue> {
    match property_name {
        "name" => resolve_property_with(contexts, field_property!(as_template_variable, name)),
        "value" => resolve_property_with(contexts, field_property!(as_template_variable, value)),
        _ => {
            unreachable!(
                "attempted to read unexpected property '{property_name}' on type 'TemplateVariable'"
            )
        }
    }
}

pub(super) fn resolve_terraform_property<'a, V: AsVertex<Vertex> + 'a>(
    contexts: ContextIterator<'a, V>,
    property_name: &str,
    _resolve_info: &ResolveInfo,
) -> ContextOutcomeIterator<'a, V, FieldValue> {
    match property_name {
        "required_version" => {
            resolve_property_with(contexts, field_property!(as_terraform, required_version))
        }
        _ => {
            unreachable!(
                "attempted to read unexpected property '{property_name}' on type 'Terraform'"
            )
        }
    }
}

pub(super) fn resolve_variable_property<'a, V: AsVertex<Vertex> + 'a>(
    contexts: ContextIterator<'a, V>,
    property_name: &str,
    _resolve_info: &ResolveInfo,
) -> ContextOutcomeIterator<'a, V, FieldValue> {
    match property_name {
        "name" => resolve_property_with(contexts, field_property!(as_variable, name)),
        "value" => resolve_property_with(contexts, field_property!(as_variable, value)),
        _ => {
            unreachable!(
                "attempted to read unexpected property '{property_name}' on type 'Variable'"
            )
        }
    }
}
