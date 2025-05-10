import dotenv from 'dotenv';
dotenv.config();

/**
 * Resources: 
 * - https://tanstack.com/query/latest/docs/framework/react/graphql
 * - https://github.com/dotansimha/graphql-code-generator/tree/master/examples/react/tanstack-react-query
 * - https://the-guild.dev/graphql/codegen/docs/guides/react-vue#appendix-i-react-query-with-a-custom-fetcher-setup
 */

// TODO: fix issue of codegen integration with graphql-config tool
export default {
  schema: "../api-data/server/config/schema-autogen.graphql",
  documents: "src/**/*.{ts,tsx,graphql,gql,js,jsx}",
};
