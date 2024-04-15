use super::utils::{AmazonApigatewayIntegration, Operator, Route};

#[non_exhaustive]
#[derive(Debug, Clone, trustfall::provider::TrustfallEnumVertex)]
pub enum Vertex {
    AmazonApigatewayIntegration(AmazonApigatewayIntegration),
    Info(openapiv3::Info),
    Operation(Operator),
    Path(Route),
    Paths(Vec<Route>),
    Tags(Vec<openapiv3::Tag>),
    Tag(openapiv3::Tag),
}
