import type { CodegenConfig } from '@graphql-codegen/cli'

import { loadConfig } from 'graphql-config';
const c = await loadConfig();

const config: CodegenConfig = {
  schema: c?.getDefault().schema,
  documents: c?.getDefault().documents,
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