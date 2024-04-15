use trustfall::provider::check_adapter_invariants;

use super::FileSystemAdapter;

#[test]
fn adapter_satisfies_trustfall_invariants() {
    let adapter = FileSystemAdapter::new();
    let schema = FileSystemAdapter::schema();
    check_adapter_invariants(schema, adapter);
}
