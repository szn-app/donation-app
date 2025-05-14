import React, { useEffect } from "react";
import { z } from "zod";
import { useAuth } from "react-oidc-context";
import { request } from "graphql-request";
import { useQuery } from "@tanstack/react-query";
import { graphql } from "@/graphql-generated/gql";

import { AccountSchema } from "@/graphql-generated/runtime-validate";
import {
  DummyQuery,
  Account,
  GetAccountListQuery,
  GetAccountListDocument,
} from "@/graphql-generated/graphql";

const DUMMY_QUERY = graphql(`
  query Dummy {
    accounts {
      id
      createdAt
    }
  }
`);

export const E = ExampleGraphqlZodParsing;

export function ExampleGraphqlZodParsing() {
  const { data, isLoading, isError, error } = useQuery({
    queryKey: ["cache-key-1"],
    queryFn: async () =>
      await request<GetAccountListQuery>(
        import.meta.env.VITE_GRAPHQL_ENDPOINT,
        GetAccountListDocument.toString(),
      ),
    // parsing setp to match expected types to returned values on runtime
    select: (raw) => ({
      accounts: raw.accounts.map((account) => {
        const { data, error, success } = AccountSchema().safeParse(account);
        if (!success) {
          console.error("Validation failed for account:", error);
          // Optionally return a fallback value or rethrow the error
          throw error;
        }
        return data;
      }),
    }),
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
    <>No items present.</>
  );
}

export function ExampleGraphqlQueryIntegratedParsing() {
  const { data, isLoading, isError, error } = useQuery<DummyQuery>({
    queryKey: ["dummy"],
    queryFn: async () => {
      const response = await request<DummyQuery>(
        import.meta.env.VITE_GRAPHQL_ENDPOINT,
        DUMMY_QUERY.toString(),
      );

      return response;
    },
    // parsing setp to match types on runtime
    select: (data) => ({
      accounts: data.accounts.map((account) => ({
        ...account,
        createdAt: new Date(account.createdAt),
      })),
    }),
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

export function ExampleGraphqlFetchManualParsing() {
  useEffect(() => {
    async function fetch_data() {
      const { accounts } = await request<DummyQuery>(
        import.meta.env.VITE_GRAPHQL_ENDPOINT,
        DUMMY_QUERY.toString(),
      );

      return accounts as Account[];
    }

    try {
      fetch_data().then((accounts) => {
        // NOTE: response still requires parsing on runtime;

        console.log(typeof accounts[0].createdAt);
        console.log(
          typeof new Date(accounts[0].createdAt as unknown as string),
        );

        let d = new Date(accounts[0].createdAt as unknown as string);
        console.log(d.toISOString());
      });
    } catch (e) {
      console.error(e);
    }

    return () => {};
  }, []);

  return <h1>END</h1>;
}

// usage example query
export function ExampleGraphqlWithoutAuth() {
  // NOTE: typescipt doesn't enforce types at runtime, thus parsing is required
  const { data, isLoading, isError, error } = useQuery<DummyQuery>({
    queryKey: ["dummy"],
    queryFn: async (): Promise<DummyQuery> => {
      const r = await request<DummyQuery>(
        import.meta.env.VITE_GRAPHQL_ENDPOINT,
        DUMMY_QUERY.toString(),
      );

      return r;
    },
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

// usage example with authorization header
export function ExampleGraphqlWIthAuth() {
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
