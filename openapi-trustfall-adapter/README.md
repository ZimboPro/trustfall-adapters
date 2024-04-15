# OpenAPI Trustfall Adapter

A OpenAPI Trustfall adapter for the v3.0.x spec.

## Spec

```graphql

type RootSchemaQuery {
    """
    OpenAPI doc info
    """
    Info: Info!
    
    """
    OpenAPI doc tags
    """
    Tags: [Tag!]!

    """
    OpenAPI doc paths
    """
    Paths: [Path!]!

    """
    OpenAPI doc path

    path: the path to the endpoint
    """
    Path(path: String!): Path

}

type Info {
    title: String!
    version: String!
    description: String
}

type Tag {
    name: String!
    description: String
}

type Path {
    path: String!
    get: Operation
    post: Operation
    put: Operation
    delete: Operation
    patch: Operation
    options: Operation
    operations: [Operation!]!
}

type Operation {
    summary: String
    method: String!
    description: String
    tags: [String!]
    xAmazonApigatewayIntegration: AmazonApigatewayIntegration
}

type AmazonApigatewayIntegration {
    type: String!
    httpMethod: String!
    uri: String!
    passthroughBehavior: String!
    timeoutInMillis: Int
    trigger: String!
    arn: String
}
```

NOTE:
The `arn` property in the `AmazonApigatewayIntegration` integration type is expecting a specific format as follows:

```yaml
x-amazon-apigateway-integration:
    ...
    uri: "arn:aws:apigateway:us-east-1:lambda:path/2015-03-31/functions/${some_service_arn}/invocations"
    ...
```
OR
```yaml
x-amazon-apigateway-integration:
    ...
    uri: "arn:aws:apigateway:${region}:lambda:path/2015-03-31/functions/${some_service_arn}/invocations"
    ...
```

It will try extract the value `some_service_arn`.