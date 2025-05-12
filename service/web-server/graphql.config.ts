import dotenv from "dotenv";
dotenv.config();

// TODO: fix issue of codegen integration with graphql-config tool
export default {
  schema: "../api-data/server/config/schema-autogen.graphql",
  documents: [
    "src/**/*.{ts,tsx,graphql,js,jsx}",
    "!src/graphql-generated/**/*",
  ],
};
