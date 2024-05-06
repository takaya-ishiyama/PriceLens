import { json } from "@remix-run/node";
import { useLoaderData, useParams } from "@remix-run/react";
import { client } from "app/infrastructure";

import {
  GetOrganizationDocument,
  GetOrganizationQuery,
  GetOrganizationQueryVariables,
} from "app/infrastructure/graphql";
import { RequestOptions } from "graphql-request";

export async function loader() {
  const params = useParams();
  const requestOptions: RequestOptions<
    GetOrganizationQueryVariables,
    GetOrganizationQuery
  > = {
    document: GetOrganizationDocument,
    variables: { id: "5cda43e9-abfe-4ddd-800c-c1a8dedb4bcf" },
  };

  const { findOne } = await client.request(requestOptions);
  return json({ findOne });
}

export const useGetOrganization = () => {
  const { findOne } = useLoaderData<typeof loader>();

  return { findOne };
};
