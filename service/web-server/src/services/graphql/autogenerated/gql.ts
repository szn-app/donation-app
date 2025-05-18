/* eslint-disable */
import * as types from './graphql';



/**
 * Map of all GraphQL operations in the project.
 *
 * This map has several performance disadvantages:
 * 1. It is not tree-shakeable, so it will include all operations in the project.
 * 2. It is not minifiable, so the string of a GraphQL query will be multiple times inside the bundle.
 * 3. It does not support dead code elimination, so it will add unused operations.
 *
 * Therefore it is highly recommended to use the babel or swc plugin for production.
 * Learn more about it here: https://the-guild.dev/graphql/codegen/plugins/presets/preset-client#reducing-bundle-size
 */
type Documents = {
    "\n  query GetTestListPartial {\n    tests {\n      i\n      d\n    }\n  }\n": typeof types.GetTestListPartialDocument,
    "query GetTestList2 {\n  tests {\n    i\n    s\n    d\n  }\n}": typeof types.GetTestList2Document,
    "query GetTestList1 {\n  tests {\n    i\n    s\n    d\n  }\n}": typeof types.GetTestList1Document,
    "query GetAccountList {\n  accounts {\n    id\n    createdAt\n  }\n}\n\nquery GetTestList {\n  tests {\n    i\n    s\n    d\n  }\n}": typeof types.GetAccountListDocument,
};
const documents: Documents = {
    "\n  query GetTestListPartial {\n    tests {\n      i\n      d\n    }\n  }\n": types.GetTestListPartialDocument,
    "query GetTestList2 {\n  tests {\n    i\n    s\n    d\n  }\n}": types.GetTestList2Document,
    "query GetTestList1 {\n  tests {\n    i\n    s\n    d\n  }\n}": types.GetTestList1Document,
    "query GetAccountList {\n  accounts {\n    id\n    createdAt\n  }\n}\n\nquery GetTestList {\n  tests {\n    i\n    s\n    d\n  }\n}": types.GetAccountListDocument,
};

/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "\n  query GetTestListPartial {\n    tests {\n      i\n      d\n    }\n  }\n"): typeof import('./graphql').GetTestListPartialDocument;
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "query GetTestList2 {\n  tests {\n    i\n    s\n    d\n  }\n}"): typeof import('./graphql').GetTestList2Document;
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "query GetTestList1 {\n  tests {\n    i\n    s\n    d\n  }\n}"): typeof import('./graphql').GetTestList1Document;
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "query GetAccountList {\n  accounts {\n    id\n    createdAt\n  }\n}\n\nquery GetTestList {\n  tests {\n    i\n    s\n    d\n  }\n}"): typeof import('./graphql').GetAccountListDocument;


export function graphql(source: string) {
  return (documents as any)[source] ?? {};
}
