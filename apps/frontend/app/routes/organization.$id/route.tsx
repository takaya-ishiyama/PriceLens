import React from "react";
import { RequestOptions, gql } from "graphql-request";
import {
  GetOrganizationQueryVariables,
  GetOrganizationQuery,
  GetOrganizationDocument,
} from "app/infrastructure/graphql";
import { client } from "app/infrastructure";
import { useLoaderData } from "@remix-run/react";
import { LoaderFunction } from "@remix-run/node";

export const loader: LoaderFunction = async ({ params }) => {
  if (params.id == null) return
  const requestOptions: RequestOptions<
    GetOrganizationQueryVariables,
    GetOrganizationQuery
  > = {
    document: GetOrganizationDocument,
    variables: { id: params.id },
    // variables: { id: "5cda43e9-abfe-4ddd-800c-c1a8dedb4bcf" },
  };
  const { findOne } = await client.request(requestOptions);
  return { organization: findOne };
};

const OrganizationScreen: React.FC = () => {
  const { organization } = useLoaderData<typeof loader>();

  console.log("\n-----------------------\n", organization);

  return (
    <div>
      <div>aaaa</div>
    </div>
  );
};
export default OrganizationScreen;
