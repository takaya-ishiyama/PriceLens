import { client } from "app/infrastructure";
import { CreateOrganizationMutation, CreateOrganizationMutationVariables, GetOrganizationDocument, OrganizationType } from "app/infrastructure/graphql";
import { RequestOptions } from "graphql-request";
import React from "react";

type Props = {
  name: string;
  organizationType: OrganizationType;
};
type CreateOrganization = {
  organization: CreateOrganizationMutation['createOrganization'];
};
export const useCreateOrganization = (props: Props) => {
  const requestOptions: RequestOptions<
    CreateOrganizationMutationVariables,
    CreateOrganizationMutation
  > = {
    document: GetOrganizationDocument,
    variables: props,
  };
  const handleSubmitCreateOrganization = React.useCallback(async () => {
    try {
      const { createOrganization } = await client.request(requestOptions);
      return { organization: createOrganization } as CreateOrganization;
    } catch (e) {
      console.log("エラーはっせい！！");
      throw new Error((e as Error).message);
    }
  }, [requestOptions])

  return { handleSubmitCreateOrganization }

};
