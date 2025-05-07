# development flow: 
- [automated script] copying .proto files and modifying import statements to flat structure
- [manual] importing Rust generated proto modules (check protobuf-autogen/packages_detected.txt) and re-exporting them

## Resource:
- [.proto files](https://github.com/ory/keto/tree/master/proto/ory/keto/relation_tuples/v1alpha2)
- Protocol buffers api definitions https://www.ory.sh/docs/keto/reference/proto-api#ory-keto-opl-v1alpha1-CheckRequest 
  - from https://github.com/ory/keto/blob/master/proto/buf.md
- SDK docs https://www.ory.sh/docs/keto/sdk/overview/
- REST API https://www.ory.sh/docs/keto/reference/rest-api#tag/permission

