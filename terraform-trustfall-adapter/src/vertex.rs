use super::model::{
    ApiConfig, Backend, Lambda, Module, Permissions, RequiredProvider, TemplateVariable, Terraform,
    Variable,
};

#[non_exhaustive]
#[derive(Debug, Clone, trustfall::provider::TrustfallEnumVertex)]
pub enum Vertex {
    ApiConfig(ApiConfig),
    Backend(Backend),
    Lambda(Lambda),
    Module(Module),
    Permissions(Permissions),
    RequiredProvider(RequiredProvider),
    TemplateVariable(TemplateVariable),
    Terraform(Terraform),
    Variable(Variable),
}
