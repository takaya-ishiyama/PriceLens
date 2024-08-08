import { useLoaderData, useNavigate } from "@remix-run/react";
import { RouteName } from "app/consts/route_name";
import React from "react";
import { useCreateOrganization } from "./hooks/useCreateOrganization";
import { Button } from "@/components/ui/button";
import { InputWithRHF } from "@/components/molecule/Input/InputWithRHF";
import { useCreateOrganizationForm } from "./hooks/useCreateOrganizationForm";
import type { LoaderFunction } from "@remix-run/node";
import type { RequestOptions } from "graphql-request";
import {
  FindManyOrganizationWithPageneDocument,
  type FindManyOrganizationWithPageneQuery,
  type FindManyOrganizationWithPageneQueryVariables,
} from "app/infrastructure/graphql";
import { client } from "app/infrastructure/graphql-request";
import { Input } from "@/components/atom";
import { NodeOrganizations } from "./components/NodeOrganizations";

export const loader: LoaderFunction = async ({ params }) => {
  const requestOptions: RequestOptions<
    FindManyOrganizationWithPageneQueryVariables,
    FindManyOrganizationWithPageneQuery
  > = {
    document: FindManyOrganizationWithPageneDocument,
    variables: { first: 10 },
  };
  try {
    const { organizationFindAllWithPagenate } =
      await client.request(requestOptions);
    return organizationFindAllWithPagenate;
  } catch (e) {
    console.log("エラーはっせい！！");
    throw new Error((e as Error).message);
  }
};

const TopScreen = () => {
  const navigate = useNavigate();
  const [organizationId] = React.useState<string | null>(
    "5cda43e9-abfe-4ddd-800c-c1a8dedb4bcf",
  );

  const {
    form: {
      handleSubmit: handleSubmitWithRHF,
      control,
      formState: { errors },
    },
  } = useCreateOrganizationForm();

  const { handleSubmitCreateOrganization } = useCreateOrganization();

  const handleSubmit = React.useCallback(
    () => handleSubmitWithRHF(handleSubmitCreateOrganization)(),
    [handleSubmitWithRHF, handleSubmitCreateOrganization],
  );


  // 今のところ使っていない
  // const handleClickGoToOrganization = React.useCallback(
  //   (organizationId: string) => {
  //     if (organizationId === null) return;
  //     navigate(RouteName.organization(organizationId));
  //   },
  //   [navigate],
  // );

  return (
    <div>
      <div>
        <Button onClick={handleSubmit}>create organizatioon</Button>
        {/* modalにする予定 */}
        <InputWithRHF name={"name"} control={control} />
      </div>
      <div>
        <Input />
        <Button>検索</Button>
      </div>
      <NodeOrganizations />
    </div>
  );
};
export default TopScreen;
