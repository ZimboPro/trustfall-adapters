# Filesystem Trustfall Adapter

A Filesystem Trustfall adapter

## Spec

```graphql
type RootSchemaQuery {
    Path(path: String!): Path!
}

interface Path {
    path: String!

}

type Folder implements Path {
    path: String!
    children: [Path!]
}

interface File implements Path {
    path: String!
    size: Int!
    extension: String!
    """
    SHA256 hash of the file
    """
    Hash: String!
}
```

## TODO

- [ ] Add examples
- [ ] improve docs
- [ ] Add tests
- [ ] Example queries