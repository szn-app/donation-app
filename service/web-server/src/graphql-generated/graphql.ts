/* eslint-disable */
import type { DocumentTypeDecoration } from '@graphql-typed-document-node/core';
export type Maybe<T> = T | null;
export type InputMaybe<T> = Maybe<T>;
export type Exact<T extends { [key: string]: unknown }> = { [K in keyof T]: T[K] };
export type MakeOptional<T, K extends keyof T> = Omit<T, K> & { [SubKey in K]?: Maybe<T[SubKey]> };
export type MakeMaybe<T, K extends keyof T> = Omit<T, K> & { [SubKey in K]: Maybe<T[SubKey]> };
export type MakeEmpty<T extends { [key: string]: unknown }, K extends keyof T> = { [_ in K]?: never };
export type Incremental<T> = T | { [P in keyof T]?: P extends ' $fragmentName' | '__typename' ? T[P] : never };
/** All built-in and custom scalars, mapped to their actual values */
export type Scalars = {
  ID: { input: string; output: string; }
  String: { input: string; output: string; }
  Boolean: { input: boolean; output: boolean; }
  Int: { input: number; output: number; }
  Float: { input: number; output: number; }
  /**
   * A datetime with timezone offset.
   *
   * The input is a string in RFC3339 format, e.g. "2022-01-12T04:00:19.12345Z"
   * or "2022-01-12T04:00:19+03:00". The output is also a string in RFC3339
   * format, but it is always normalized to the UTC (Z) offset, e.g.
   * "2022-01-12T04:00:19.12345Z".
   */
  DateTime: { input: string; output: Date; }
  /**
   * A UUID is a unique 128-bit number, stored as 16 octets. UUIDs are parsed as
   * Strings within GraphQL. UUIDs are used to assign unique identifiers to
   * entities without requiring a central allocating authority.
   *
   * # References
   *
   * * [Wikipedia: Universally Unique Identifier](http://en.wikipedia.org/wiki/Universally_unique_identifier)
   * * [RFC4122: A Universally Unique IDentifier (UUID) URN Namespace](http://tools.ietf.org/html/rfc4122)
   */
  UUID: { input: string; output: string; }
};

export type Account = {
  __typename?: 'Account';
  createdAt: Scalars['DateTime']['output'];
  /** kratos id value */
  id: Scalars['UUID']['output'];
};

export type Mutation = {
  __typename?: 'Mutation';
  /** Create a new user */
  addAccount: Account;
};


export type MutationAddAccountArgs = {
  id: Scalars['UUID']['input'];
};

export type Query = {
  __typename?: 'Query';
  /** Get all accounts */
  accounts: Array<Account>;
  dummyTest: Array<Scalars['String']['output']>;
  dummyTestRequestHeader: Scalars['String']['output'];
  dummyTestSecure: Test;
  dummyTestSecureGuard: Test;
  dummyTestSecurePermissionCheck: Test;
  tests: Array<Test>;
};

export type Test = {
  __typename?: 'Test';
  d: Scalars['DateTime']['output'];
  /** example graphql comments (should appear in graphql IDE) */
  i: Scalars['Int']['output'];
  s: Scalars['String']['output'];
};

export type GetTestListPartialQueryVariables = Exact<{ [key: string]: never; }>;


export type GetTestListPartialQuery = { __typename?: 'Query', tests: Array<{ __typename?: 'Test', i: number, d: Date }> };

export type DummyQueryVariables = Exact<{ [key: string]: never; }>;


export type DummyQuery = { __typename: 'Query' };

export type GetAccountListQueryVariables = Exact<{ [key: string]: never; }>;


export type GetAccountListQuery = { __typename?: 'Query', accounts: Array<{ __typename?: 'Account', id: string, createdAt: Date }> };

export type GetTestListQueryVariables = Exact<{ [key: string]: never; }>;


export type GetTestListQuery = { __typename?: 'Query', tests: Array<{ __typename?: 'Test', i: number, s: string, d: Date }> };

export class TypedDocumentString<TResult, TVariables>
  extends String
  implements DocumentTypeDecoration<TResult, TVariables>
{
  __apiType?: DocumentTypeDecoration<TResult, TVariables>['__apiType'];
  private value: string;
  public __meta__?: Record<string, any> | undefined;

  constructor(value: string, __meta__?: Record<string, any> | undefined) {
    super(value);
    this.value = value;
    this.__meta__ = __meta__;
  }

  toString(): string & DocumentTypeDecoration<TResult, TVariables> {
    return this.value;
  }
}

export const GetTestListPartialDocument = new TypedDocumentString(`
    query GetTestListPartial {
  tests {
    i
    d
  }
}
    `) as unknown as TypedDocumentString<GetTestListPartialQuery, GetTestListPartialQueryVariables>;
export const DummyDocument = new TypedDocumentString(`
    query Dummy {
  __typename
}
    `) as unknown as TypedDocumentString<DummyQuery, DummyQueryVariables>;
export const GetAccountListDocument = new TypedDocumentString(`
    query GetAccountList {
  accounts {
    id
    createdAt
  }
}
    `) as unknown as TypedDocumentString<GetAccountListQuery, GetAccountListQueryVariables>;
export const GetTestListDocument = new TypedDocumentString(`
    query GetTestList {
  tests {
    i
    s
    d
  }
}
    `) as unknown as TypedDocumentString<GetTestListQuery, GetTestListQueryVariables>;