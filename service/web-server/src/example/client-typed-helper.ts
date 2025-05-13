/* eslint-disable import/no-extraneous-dependencies */
import {
  useQuery,
  UseQueryOptions,
  UseQueryResult,
} from "@tanstack/react-query";
import { TypedDocumentString } from "@/graphql-generated/graphql";
import { request, RequestDocument, GraphQLClient } from "graphql-request";
import { useAuth } from "react-oidc-context";

/**
 * Custom hook to create a GraphQL client instance with optional auth headers
 * based on the current user's access token from OIDC context.
 *
 * @returns Configured graphql-request GraphQLClient instance
 */
export function createGraphQLClient(token?: string): GraphQLClient {
  const headers = token ? { Authorization: `Bearer ${token}` } : undefined;
  const endpoint = import.meta.env.VITE_GRAPHQL_ENDPOINT;

  return new GraphQLClient(endpoint, { headers });
}

// Example usage:
// const { data, isLoading, error } = useGraphQL(
//   GET_USER_QUERY,
//   { id: userId },
//   { enabled: !!userId }
// );
/**
 * Type-safe wrapper for useQuery with GraphQL
 * https://github.com/dotansimha/graphql-code-generator/blob/master/examples/react/tanstack-react-query/src/use-graphql.ts
 * https://the-guild.dev/graphql/codegen/docs/guides/react-query#type-safe-graphql-operation-execution
 *
 * @param document - GraphQL typed document string
 * @param variables - Query variables (optional if no variables needed)
 * @param options - Additional TanStack Query options
 * @returns Query result with data, loading state, and error handling
 */
export function useGraphQL<TResult, TVariables extends object>(
  document: TypedDocumentString<TResult, TVariables>,
  variables: TVariables = {} as TVariables, // Default to empty object
  options?: Omit<
    UseQueryOptions<TResult, Error, TResult, readonly [string, TVariables]>,
    "queryKey" | "queryFn"
  >,
): UseQueryResult<TResult, Error> {
  const client = createGraphQLClient();

  const requestDocument = document as unknown as RequestDocument;

  // https://github.com/graffle-js/graffle
  const queryFn = async () =>
    await client.request<TResult>(requestDocument, {}, {});

  const queryKey = [document.toString(), variables] as const;

  return useQuery({
    queryKey,
    queryFn,
    ...options,
  });
}
