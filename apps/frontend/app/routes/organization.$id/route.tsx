import React from "react";
import { RequestOptions, gql } from "graphql-request";
import { GetOrganizationDocument, GetOrganizationQueryVariables, GetOrganizationQuery, } from "app/infrastructure/graphql";
import { client } from "app/infrastructure";
import { json } from "@remix-run/node";
import { useLoaderData, useParams } from "@remix-run/react";
import { useGetOrganization } from "./hooks";

const query = gql`
query getOrganization($id: ID!){
  findOne(id: $id){
    id
    name
  }
} 
`
export async function loader() {
  const requestOptions: RequestOptions<
    GetOrganizationQueryVariables,
    GetOrganizationQuery
  > = {
    document: GetOrganizationDocument,
    variables: { id: "5cda43e9-abfe-4ddd-800c-c1a8dedb4bcf" },
  };
  const params = useParams();
  const { findOne } = await client.request(requestOptions);
  return { organization: findOne }
};


const OrganizationScreen: React.FC = () => {
  const { organization } = useLoaderData<typeof loader>();

  return (
    <div>
      <div>aaaa</div >
    </div>
  )
}
export default OrganizationScreen;
