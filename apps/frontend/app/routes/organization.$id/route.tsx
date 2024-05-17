import React from "react";
import { RequestOptions } from "graphql-request";
import {
  GetOrganizationQueryVariables,
  GetOrganizationQuery,
  GetOrganizationDocument,
} from "app/infrastructure/graphql";
import { client } from "app/infrastructure";
import { useLoaderData } from "@remix-run/react";
import { LoaderFunction } from "@remix-run/node";

type OrganizationQuery = { organization: GetOrganizationQuery['findOne'] }

export const loader: LoaderFunction = async ({ params }) => {
  if (params.id == null) return
  const requestOptions: RequestOptions<
    GetOrganizationQueryVariables,
    GetOrganizationQuery
  > = {
    document: GetOrganizationDocument,
    variables: { id: params.id },
  };
  try {
    const { findOne } = await client.request(requestOptions);
    return { organization: findOne } as OrganizationQuery;
  } catch (e) {
    console.log('エラーはっせい！！')
    throw new Error((e as Error).message)
  }
};

const OrganizationScreen: React.FC = () => {
  const { organization } = useLoaderData<OrganizationQuery>();

  return (
    <div>
      <div>
        {organization.name}
      </div>
      <div>
        {organization.organizationType}
      </div>
    </div>
  );
};
export default OrganizationScreen;
