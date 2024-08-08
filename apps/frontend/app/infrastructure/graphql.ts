/* eslint-disable */
import { TypedDocumentNode as DocumentNode } from '@graphql-typed-document-node/core';
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
};

export type ItemSchema = {
  __typename?: 'ItemSchema';
  id: Scalars['String']['output'];
  name: Scalars['String']['output'];
  organizationId: Scalars['String']['output'];
};

export type Mutation = {
  __typename?: 'Mutation';
  createItem: ItemSchema;
  createOrganization: OrganizationSchema;
};


export type MutationCreateItemArgs = {
  name: Scalars['String']['input'];
  organizationId: Scalars['String']['input'];
};


export type MutationCreateOrganizationArgs = {
  name: Scalars['String']['input'];
  organizationType: Scalars['String']['input'];
  privateKey?: InputMaybe<Scalars['String']['input']>;
};

export type OrganizationSchema = {
  __typename?: 'OrganizationSchema';
  id: Scalars['String']['output'];
  name: Scalars['String']['output'];
  organizationType: OrganizationType;
};

export type OrganizationSchemaConnection = {
  __typename?: 'OrganizationSchemaConnection';
  /** A list of edges. */
  edges: Array<OrganizationSchemaEdge>;
  /** A list of nodes. */
  nodes: Array<OrganizationSchema>;
  /** Information to aid in pagination. */
  pageInfo: PageInfo;
};

/** An edge in a connection. */
export type OrganizationSchemaEdge = {
  __typename?: 'OrganizationSchemaEdge';
  /** A cursor for use in pagination */
  cursor: Scalars['String']['output'];
  /** The item at the end of the edge */
  node: OrganizationSchema;
};

export enum OrganizationType {
  Private = 'PRIVATE',
  Public = 'PUBLIC'
}

/** Information about pagination in a connection */
export type PageInfo = {
  __typename?: 'PageInfo';
  /** When paginating forwards, the cursor to continue. */
  endCursor?: Maybe<Scalars['String']['output']>;
  /** When paginating forwards, are there more items? */
  hasNextPage: Scalars['Boolean']['output'];
  /** When paginating backwards, are there more items? */
  hasPreviousPage: Scalars['Boolean']['output'];
  /** When paginating backwards, the cursor to continue. */
  startCursor?: Maybe<Scalars['String']['output']>;
};

export type Query = {
  __typename?: 'Query';
  currentToken?: Maybe<Scalars['String']['output']>;
  organizationFindAllWithPagenate: OrganizationSchemaConnection;
  organizationFindManyByName: Array<OrganizationSchema>;
  organizationFindOne: OrganizationSchema;
};


export type QueryOrganizationFindAllWithPagenateArgs = {
  after?: InputMaybe<Scalars['String']['input']>;
  before?: InputMaybe<Scalars['String']['input']>;
  first?: InputMaybe<Scalars['Int']['input']>;
  last?: InputMaybe<Scalars['Int']['input']>;
};


export type QueryOrganizationFindManyByNameArgs = {
  name: Scalars['String']['input'];
};


export type QueryOrganizationFindOneArgs = {
  id: Scalars['ID']['input'];
};

export type GetOrganizationQueryVariables = Exact<{
  id: Scalars['ID']['input'];
}>;


export type GetOrganizationQuery = { __typename?: 'Query', organizationFindOne: { __typename?: 'OrganizationSchema', id: string, name: string, organizationType: OrganizationType } };

export type CreateOrganizationMutationVariables = Exact<{
  name: Scalars['String']['input'];
  organizationType: Scalars['String']['input'];
  privateKey?: InputMaybe<Scalars['String']['input']>;
}>;


export type CreateOrganizationMutation = { __typename?: 'Mutation', createOrganization: { __typename?: 'OrganizationSchema', id: string, name: string, organizationType: OrganizationType } };

export type FindManyOrganizationWithPageneQueryVariables = Exact<{
  first?: InputMaybe<Scalars['Int']['input']>;
  last?: InputMaybe<Scalars['Int']['input']>;
  after?: InputMaybe<Scalars['String']['input']>;
  before?: InputMaybe<Scalars['String']['input']>;
}>;


export type FindManyOrganizationWithPageneQuery = { __typename?: 'Query', organizationFindAllWithPagenate: { __typename?: 'OrganizationSchemaConnection', nodes: Array<{ __typename?: 'OrganizationSchema', id: string, organizationType: OrganizationType, name: string }>, pageInfo: { __typename?: 'PageInfo', hasNextPage: boolean, hasPreviousPage: boolean } } };


export const GetOrganizationDocument = {"kind":"Document","definitions":[{"kind":"OperationDefinition","operation":"query","name":{"kind":"Name","value":"getOrganization"},"variableDefinitions":[{"kind":"VariableDefinition","variable":{"kind":"Variable","name":{"kind":"Name","value":"id"}},"type":{"kind":"NonNullType","type":{"kind":"NamedType","name":{"kind":"Name","value":"ID"}}}}],"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"organizationFindOne"},"arguments":[{"kind":"Argument","name":{"kind":"Name","value":"id"},"value":{"kind":"Variable","name":{"kind":"Name","value":"id"}}}],"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"id"}},{"kind":"Field","name":{"kind":"Name","value":"name"}},{"kind":"Field","name":{"kind":"Name","value":"organizationType"}}]}}]}}]} as unknown as DocumentNode<GetOrganizationQuery, GetOrganizationQueryVariables>;
export const CreateOrganizationDocument = {"kind":"Document","definitions":[{"kind":"OperationDefinition","operation":"mutation","name":{"kind":"Name","value":"createOrganization"},"variableDefinitions":[{"kind":"VariableDefinition","variable":{"kind":"Variable","name":{"kind":"Name","value":"name"}},"type":{"kind":"NonNullType","type":{"kind":"NamedType","name":{"kind":"Name","value":"String"}}}},{"kind":"VariableDefinition","variable":{"kind":"Variable","name":{"kind":"Name","value":"organizationType"}},"type":{"kind":"NonNullType","type":{"kind":"NamedType","name":{"kind":"Name","value":"String"}}}},{"kind":"VariableDefinition","variable":{"kind":"Variable","name":{"kind":"Name","value":"privateKey"}},"type":{"kind":"NamedType","name":{"kind":"Name","value":"String"}}}],"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"createOrganization"},"arguments":[{"kind":"Argument","name":{"kind":"Name","value":"name"},"value":{"kind":"Variable","name":{"kind":"Name","value":"name"}}},{"kind":"Argument","name":{"kind":"Name","value":"organizationType"},"value":{"kind":"Variable","name":{"kind":"Name","value":"organizationType"}}},{"kind":"Argument","name":{"kind":"Name","value":"privateKey"},"value":{"kind":"Variable","name":{"kind":"Name","value":"privateKey"}}}],"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"id"}},{"kind":"Field","name":{"kind":"Name","value":"name"}},{"kind":"Field","name":{"kind":"Name","value":"organizationType"}}]}}]}}]} as unknown as DocumentNode<CreateOrganizationMutation, CreateOrganizationMutationVariables>;
export const FindManyOrganizationWithPageneDocument = {"kind":"Document","definitions":[{"kind":"OperationDefinition","operation":"query","name":{"kind":"Name","value":"findManyOrganizationWithPagene"},"variableDefinitions":[{"kind":"VariableDefinition","variable":{"kind":"Variable","name":{"kind":"Name","value":"first"}},"type":{"kind":"NamedType","name":{"kind":"Name","value":"Int"}}},{"kind":"VariableDefinition","variable":{"kind":"Variable","name":{"kind":"Name","value":"last"}},"type":{"kind":"NamedType","name":{"kind":"Name","value":"Int"}}},{"kind":"VariableDefinition","variable":{"kind":"Variable","name":{"kind":"Name","value":"after"}},"type":{"kind":"NamedType","name":{"kind":"Name","value":"String"}}},{"kind":"VariableDefinition","variable":{"kind":"Variable","name":{"kind":"Name","value":"before"}},"type":{"kind":"NamedType","name":{"kind":"Name","value":"String"}}}],"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"organizationFindAllWithPagenate"},"arguments":[{"kind":"Argument","name":{"kind":"Name","value":"first"},"value":{"kind":"Variable","name":{"kind":"Name","value":"first"}}},{"kind":"Argument","name":{"kind":"Name","value":"last"},"value":{"kind":"Variable","name":{"kind":"Name","value":"last"}}},{"kind":"Argument","name":{"kind":"Name","value":"after"},"value":{"kind":"Variable","name":{"kind":"Name","value":"after"}}},{"kind":"Argument","name":{"kind":"Name","value":"before"},"value":{"kind":"Variable","name":{"kind":"Name","value":"before"}}}],"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"nodes"},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"id"}},{"kind":"Field","name":{"kind":"Name","value":"organizationType"}},{"kind":"Field","name":{"kind":"Name","value":"name"}}]}},{"kind":"Field","name":{"kind":"Name","value":"pageInfo"},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"hasNextPage"}},{"kind":"Field","name":{"kind":"Name","value":"hasPreviousPage"}}]}}]}}]}}]} as unknown as DocumentNode<FindManyOrganizationWithPageneQuery, FindManyOrganizationWithPageneQueryVariables>;