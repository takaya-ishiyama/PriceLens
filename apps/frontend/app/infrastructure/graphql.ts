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

export enum OrganizationType {
  Private = 'PRIVATE',
  Public = 'PUBLIC'
}

export type Query = {
  __typename?: 'Query';
  currentToken?: Maybe<Scalars['String']['output']>;
  organizationFindManyByName: Array<OrganizationSchema>;
  organizationFindOne: OrganizationSchema;
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


export const GetOrganizationDocument = {"kind":"Document","definitions":[{"kind":"OperationDefinition","operation":"query","name":{"kind":"Name","value":"getOrganization"},"variableDefinitions":[{"kind":"VariableDefinition","variable":{"kind":"Variable","name":{"kind":"Name","value":"id"}},"type":{"kind":"NonNullType","type":{"kind":"NamedType","name":{"kind":"Name","value":"ID"}}}}],"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"organizationFindOne"},"arguments":[{"kind":"Argument","name":{"kind":"Name","value":"id"},"value":{"kind":"Variable","name":{"kind":"Name","value":"id"}}}],"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"id"}},{"kind":"Field","name":{"kind":"Name","value":"name"}},{"kind":"Field","name":{"kind":"Name","value":"organizationType"}}]}}]}}]} as unknown as DocumentNode<GetOrganizationQuery, GetOrganizationQueryVariables>;
export const CreateOrganizationDocument = {"kind":"Document","definitions":[{"kind":"OperationDefinition","operation":"mutation","name":{"kind":"Name","value":"createOrganization"},"variableDefinitions":[{"kind":"VariableDefinition","variable":{"kind":"Variable","name":{"kind":"Name","value":"name"}},"type":{"kind":"NonNullType","type":{"kind":"NamedType","name":{"kind":"Name","value":"String"}}}},{"kind":"VariableDefinition","variable":{"kind":"Variable","name":{"kind":"Name","value":"organizationType"}},"type":{"kind":"NonNullType","type":{"kind":"NamedType","name":{"kind":"Name","value":"String"}}}},{"kind":"VariableDefinition","variable":{"kind":"Variable","name":{"kind":"Name","value":"privateKey"}},"type":{"kind":"NamedType","name":{"kind":"Name","value":"String"}}}],"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"createOrganization"},"arguments":[{"kind":"Argument","name":{"kind":"Name","value":"name"},"value":{"kind":"Variable","name":{"kind":"Name","value":"name"}}},{"kind":"Argument","name":{"kind":"Name","value":"organizationType"},"value":{"kind":"Variable","name":{"kind":"Name","value":"organizationType"}}},{"kind":"Argument","name":{"kind":"Name","value":"privateKey"},"value":{"kind":"Variable","name":{"kind":"Name","value":"privateKey"}}}],"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"id"}},{"kind":"Field","name":{"kind":"Name","value":"name"}},{"kind":"Field","name":{"kind":"Name","value":"organizationType"}}]}}]}}]} as unknown as DocumentNode<CreateOrganizationMutation, CreateOrganizationMutationVariables>;