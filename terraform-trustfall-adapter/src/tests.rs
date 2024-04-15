use trustfall::provider::check_adapter_invariants;

use super::HclAdapter;

#[test]
fn adapter_satisfies_trustfall_invariants() {
    let adapter = HclAdapter::default();
    let schema = HclAdapter::schema();
    check_adapter_invariants(schema, adapter);
}
