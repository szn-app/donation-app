Resource: 
- GraphQL argument options https://www.gatsbyjs.com/docs/graphql-reference/
- built-in graphql scalar types https://spec.graphql.org/October2021/#sec-Scalars.Built-in-Scalars
- Rust-to-Graphql type mapping: 
  - There doesn't seem to be a single reference for type conversion by async-graphql; but most likely would follow at least similar scalar mapping to other tools https://graphql-rust.github.io/juniper/master/types/scalars.html 
  - refernece from code of parsing types to values: https://github.com/async-graphql/async-graphql/tree/3001a02f6df29e259900cd85230547c2923dfcf3/src/types/external
- [#graphql(...)] attribute field to the attribute macro #[Object] or to derive macro #[derive(SimpleObject)] chaning the behavior of blocks or implements traits on structs.