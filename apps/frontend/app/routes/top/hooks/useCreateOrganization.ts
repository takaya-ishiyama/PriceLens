import {
  CreateOrganizationDocument,
  type CreateOrganizationMutation,
  type CreateOrganizationMutationVariables,
  OrganizationType,
} from "app/infrastructure/graphql";
import { GraphQLClient, type RequestOptions } from "graphql-request";
import React from "react";
import { useNavigate } from "@remix-run/react";
import { RouteName } from "app/consts/route_name";
import { client } from "app/infrastructure/graphql-request";
import { getEnvironment } from "app/infrastructure/dotenv";

type InputProps = {
  name: string;
  // organizationType: OrganizationType; //FIXME: 追加する
};

export async function action({
  request,
}: ActionFunctionArgs) {
  const { BACKEND_URI } = getEnvironment()
  const client = new GraphQLClient(BACKEND_URI);
}

export const useCreateOrganization = () => {
  const navigate = useNavigate();
  const handleSubmitCreateOrganization = React.useCallback(
    async (input: InputProps) => {
      const requestOptions: RequestOptions<
        CreateOrganizationMutationVariables,
        CreateOrganizationMutation
      > = {
        document: CreateOrganizationDocument,
        variables: { organizationType: OrganizationType.Public, ...input },
      };

      try {
        const { createOrganization } = await client.request(requestOptions);
        navigate(RouteName.organization(createOrganization.id));
      } catch (e) {
        console.log("エラーはっせい！！", (e as Error).message);
        throw new Error((e as Error).message);
      }
    },
    [navigate],
  );

  return { handleSubmitCreateOrganization };
};
