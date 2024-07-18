import { client } from "app/infrastructure";
import { CreateOrganizationDocument, CreateOrganizationMutation, CreateOrganizationMutationVariables, GetOrganizationDocument, OrganizationType } from "app/infrastructure/graphql";
import { RequestOptions } from "graphql-request";
import React from "react";
import { useNavigate } from "@remix-run/react";
import { RouteName } from "app/consts/route_name";


type InputProps = {
  name: string;
  // organizationType: OrganizationType; //FIXME: 追加する
};

type CreateOrganization = {
  organization: CreateOrganizationMutation['createOrganization'];
};
export const useCreateOrganization = () => {
  const navigate = useNavigate();
  const handleSubmitCreateOrganization = React.useCallback(async (input: InputProps) => {
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
      console.log("エラーはっせい！！");
      throw new Error((e as Error).message);
    }
  }, [OrganizationType, RouteName])

  return { handleSubmitCreateOrganization }

};
