import type React from "react";
import type { RequestOptions } from "graphql-request";
import { useLoaderData } from "@remix-run/react";
import type { HeadersFunction, LoaderFunction } from "@remix-run/node";
import {
  GetOrganizationDocument,
  type GetOrganizationQuery,
  type GetOrganizationQueryVariables,
} from "app/infrastructure/graphql";
import { client } from "app/infrastructure/graphql-request";

type OrganizationQuery = {
  organization: GetOrganizationQuery["organizationFindOne"];
};

export const headers: HeadersFunction = ({ loaderHeaders }) => {
  return {
    "Cache-Control": "stale-while-revalidate=360, max-age=3600",
  };
};

export const loader: LoaderFunction = async ({ params }) => {
  if (params.id == null) return;
  const requestOptions: RequestOptions<
    GetOrganizationQueryVariables,
    GetOrganizationQuery
  > = {
    document: GetOrganizationDocument,
    variables: { id: params.id },
  };
  try {
    const { organizationFindOne } = await client.request(requestOptions);
    return { organization: organizationFindOne } as OrganizationQuery;
  } catch (e) {
    console.log("エラーはっせい！！");
    throw new Error((e as Error).message);
  }
};

const OrganizationScreen: React.FC = () => {
  const { organization } = useLoaderData<OrganizationQuery>();

  return (
    <div>
      <div>{organization.name}</div>
      <div>{organization.organizationType}</div>
    </div>
  );
};
export default OrganizationScreen;
