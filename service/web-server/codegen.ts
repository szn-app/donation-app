import type { CodegenConfig } from "@graphql-codegen/cli";
import { loadConfig } from "graphql-config";
import { z } from "zod";

const c = await loadConfig();

const config: CodegenConfig = {
  schema: c?.getDefault().schema,
  documents: c?.getDefault().documents,
  overwrite: true,
  ignoreNoDocuments: true,
  // root-level config will apply to all generates enteries
  config: {
    // handles static type mappings (without parsing) mapping graphql objects to typescript types
    scalars: {
      UUID: "string",
      ID: "string",
      DateTime: {
        input: "string",
        output: "Date",
      },
      JSON: "{ [key: string]: any }",
      Decimal: "number", // If you have decimals in your schema
      BigInt: "number", // BigInt can be handled as number (or string depending on your use case)
      URL: "string", // URL types can be converted to strings
    },
  },
  generates: {
    // if schema is provided as endpoint then schema-ast plugin will generate the graphql schema
    "./config/schema-autogen.graphql": {
      plugins: ["schema-ast"],
      // https://the-guild.dev/graphql/codegen/plugins/other/schema-ast
      config: {
        includeDirectives: true,
      },
    },

    /**
     * - https://the-guild.dev/graphql/codegen/docs/guides/react-vue
     *
     * uses graphql-request as fetcher by default
     */
    "src/graphql-generated/": {
      preset: "client",
      // https://the-guild.dev/graphql/codegen/plugins/presets/preset-client#documentmode
      config: {
        documentMode: "string", // generated function returns a wrapper over string
        // documentMode: "document", // generated function returns an AST // not required with graphql-request setup (causes issues for somereason 'gql is not defined' in generated code)

        // https://the-guild.dev/graphql/codegen/plugins/presets/preset-client
        useTypeImports: true,
      },
    },

    // NOTE: there is currently an issue with client-present, thus need to split the config separately
    //
    // https://the-guild.dev/graphql/codegen/plugins/typescript/typescript-validation-schema
    // https://github.com/colinhacks/zod
    //
    // issue: not generating schemas for queries only inputs and data object models https://github.com/Code-Hex/graphql-codegen-typescript-validation-schema/issues/472
    // TODO: fix zod Date generation
    // "src/graphql-generated/runtime-validate.ts": {
    //   plugins: [
    //     // "typescript", // not needed because already taken care of by preset-client
    //     "typescript-validation-schema",
    //   ],
    //   config: {
    //     schema: "zod",
    //     importFrom: "src/graphql-generated/graphql.ts",
    //     withObjectType: true,
    //     // see: https://www.graphql-code-generator.com/plugins/typescript
    //     strictScalars: true,
    //     // Overrides built-in ID scalar to both input and output types as string.
    //     // see: https://the-guild.dev/graphql/codegen/plugins/typescript/typescript#scalars
    //     // You can also write the config for this plugin together
    //     scalarSchemas: {
    //       // DateTime: {
    //       //   schema: "z.string().datetime().transform((val) => new Date(val))",
    //       //   input: "z.date()",
    //       //   output: "z.date()",
    //       // },
    //     },
    //   },
    // },

    /*
    // older approach - using preset client is now recommended instead;
    // generates in typescript: graphql data object types, graphql operations types, graphql document queries, tanstack query + graphql-request react hooks for the queries
    "src/graphql-generated/sdk.ts": {
      // generates SDK for graphql-request in addition to types
      plugins: [
        "typescript",
        "typescript-operations",
        "typescript-react-query",
      ],
      config: {
        documentMode: "string",
        fetcher: "graphql-request",
        reactQueryVersion: 5,
      },
    }, 
    */
  },
  hooks: {
    // afterAllFileWrite: ["prettier --write"]
  },
};

export default config;
