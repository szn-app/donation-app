import type { CodegenConfig } from "@graphql-codegen/cli";
import { loadConfig } from "graphql-config";

const c = await loadConfig();

const config: CodegenConfig = {
  schema: c?.getDefault().schema,
  documents: c?.getDefault().documents,
  overwrite: true,
  ignoreNoDocuments: true,
  generates: {
    /**
     * - https://the-guild.dev/graphql/codegen/docs/guides/react-vue
     *
     * uses graphql-request as fetcher by default
     */
    "src/graphql-generated/": {
      preset: "client",
      useTypeImports: true, // To avoid bundling Date type inline, and keep typings clean
      // generates SDK for graphql-request in addition to types
      plugins: [
        "typescript",
        "typescript-operations",
        "typescript-react-query",
      ],
      config: {
        documentMode: "string",
        fetcher: "graphql-request",
        // handles static type mappings (without parsing) mapping graphql objects to typescript types
        scalars: {
          UUID: "string",
          ID: {
            input: "string",
            output: "string | number",
          },
          DateTime: "Date",
          JSON: "{ [key: string]: any }",
          Decimal: "number", // If you have decimals in your schema
          BigInt: "number", // BigInt can be handled as number (or string depending on your use case)
          URL: "string", // URL types can be converted to strings
        },
      },
    },
    // if schema is provided as endpoint then schema-ast plugin will generate the graphql schema
    "./config/schema-autogen.graphql": {
      plugins: ["schema-ast"],
      config: {
        includeDirectives: true,
      },
    },
  },
  hooks: {
    // afterAllFileWrite: ["prettier --write"]
  },
};

export default config;
