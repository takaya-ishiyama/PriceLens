import {
  CreateOrganizationDocument,
  type CreateOrganizationMutation,
  type CreateOrganizationMutationVariables,
  OrganizationType,
} from "app/infrastructure/graphql";
import type { RequestOptions } from "graphql-request";
import React from "react";
import { useNavigate } from "@remix-run/react";
import { RouteName } from "app/consts/route_name";
import { client } from "app/infrastructure/graphql-request";

type InputProps = {
  name: string;
  // organizationType: OrganizationType; //FIXME: 追加する
};

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
