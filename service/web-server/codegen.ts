import type { CodegenConfig } from '@graphql-codegen/cli'
 
const config: CodegenConfig = {
  schema: "../api-data/server/config/schema-autogen.graphql",
  documents: ['src/**/*.tsx'],
  overwrite: true,
  ignoreNoDocuments: true,
  generates: {
    "src/graphql-generated/": {
      preset: 'client',
      config: {
        documentMode: 'string'
      }
    },
    './config/schema-autogen.graphql': {
      plugins: ['schema-ast'],
      config: {
        includeDirectives: true
      }
    }
  }
}
 
export default config