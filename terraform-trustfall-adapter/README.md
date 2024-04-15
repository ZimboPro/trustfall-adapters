# Terraform Trustfall Adapter

A Terraform Trustfall adapter.

## Spec

```graphql
type RootSchemaQuery {
  Modules: [Module!]!
  ApiConfig: ApiConfig
  Lambda: [Lambda!]
  Module(name: String!, tag: String): [Module],
  Terraform: [Terraform!]!
}

type Module {
  source: String!
  version: String!
  variables: [Variable!]
}

type Variable {
  name: String!
  """
  It will be the string implementation of the value.
  """
  value: String!
}

type Terraform {
  required_version: String
  backend: Backend
  required_providers: [RequiredProvider!]
}

type Backend {
  name: String!
}

type RequiredProvider {
  name: String!
  source: String!
  version: String!
}

type ApiConfig {
  source: String!
  version: String!
  template_file: String!
  template_variables: [TemplateVariable!]!
}

type TemplateVariable {
  name: String!
  value: String!
  lambda: Lambda
}

type Lambda {
  name: String!
  description: String!
  handler: String!
  permissions: [Permissions!]!
}

type Permissions {
  statement_id: String!
  principal: String!
  source_arn: String!
  http_method: String
  http_path: String
}
```

## TODO

- [ ] improve docs
- [ ] add more tests
- [ ] add examples
- [ ] explanation on expected structure
