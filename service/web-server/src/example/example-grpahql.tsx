import React from "react";
import { useQuery } from "@tanstack/react-query";
import { request } from "graphql-request";
import {
  Dummy2Query,
  DummyDocument,
  DummyQuery,
} from "@/graphql-generated/graphql";
import { graphql } from "@/graphql-generated";
import { useAuth } from "react-oidc-context";

const DUMMY_QUERY = graphql(`
  query Dummy {
    accounts {
      id
      createdAt
    }
  }
`);

// usage example with authorization header
export function ExampleGraphql() {
  const auth = useAuth();
  const token = auth?.user?.access_token;

  const headers: Record<string, string> | undefined = token
    ? { Authorization: `Bearer ${token}` }
    : undefined;

  const { data, isLoading, isError, error } = useQuery<DummyQuery>({
    queryKey: ["dummy"],
    queryFn: async () =>
      request<DummyQuery>(
        import.meta.env.VITE_GRAPHQL_ENDPOINT,
        DUMMY_QUERY.toString(),
        undefined,
        headers,
      ),
  });

  if (isLoading) return <p>Loading...</p>;
  if (isError) return <p>Error: {error.message} </p>;

  const accounts = data?.accounts;

  return accounts ? (
    <div>
      <h1>Accounts list:</h1>
      <ul>
        {accounts.map((account, i) => (
          <li key={i}>
            <strong>{account.id}</strong> — {account.createdAt.toISOString()}
          </li>
        ))}
      </ul>
    </div>
  ) : (
    <>Loading ...</>
  );
}
