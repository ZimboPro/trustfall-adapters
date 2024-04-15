use trustfall::provider::check_adapter_invariants;

use crate::OpenApiAdapter;

#[test]
fn adapter_satisfies_trustfall_invariants() {
    let adapter = OpenApiAdapter::default();
    let schema = OpenApiAdapter::schema();
    check_adapter_invariants(schema, adapter);
}
