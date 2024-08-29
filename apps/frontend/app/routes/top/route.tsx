// @deprecated このファイルは削除予定

import { useLoaderData, useNavigate } from "@remix-run/react";
import { RouteName } from "app/consts/route_name";
import React, { useCallback } from "react";
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

const getOrganizations = async (after?: string) => {
  const requestOptions: RequestOptions<
    FindManyOrganizationWithPageneQueryVariables,
    FindManyOrganizationWithPageneQuery
  > = {
    document: FindManyOrganizationWithPageneDocument,
    variables: { first: 10, after },
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

export const loader: LoaderFunction = async () => {
  return getOrganizations();
};

const TopScreen = () => {
  const navigate = useNavigate();

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

  // FIXME: パラメータ変えて利フェッチ方法が見つからなかった
  const { nodes, pageInfo } =
    useLoaderData<
      FindManyOrganizationWithPageneQuery["organizationFindAllWithPagenate"]
    >();

  const [ognzInfo, setOgnzInfo] =
    React.useState<
      FindManyOrganizationWithPageneQuery["organizationFindAllWithPagenate"]
    >({ nodes, pageInfo });
  const [after, setAfter] = React.useState<string | undefined>(nodes.at(-1)?.id);

  // // 初回読み込み
  // React.useEffect(() => {
  //   getOrganizations({})
  //     .then((res) => {
  //       setOgnzInfo(res);
  //       setAfter(res.nodes.at(-1)?.id);
  //     })
  // }, []);


  const handleClickGetNextOgnz = useCallback(async () => {
    const ognz = await getOrganizations(after);
    setOgnzInfo(ognz);
    if (ognz.nodes.at(-1)?.id)
      setAfter(ognz.nodes.at(-1)?.id);
  }, [after]);

  return (
    <div>
      <div>
        <Button onClick={handleSubmit}>create organizatioon</Button>
        {/* modalにする予定 */}
        <InputWithRHF name={"name"} control={control} />
      </div>
      <div>
        {/* FIXME:検索機能作る */}
        <Input />
        <Button>検索</Button>
      </div>
      <NodeOrganizations onClickGotoNextPage={handleSubmit} nodes={ognzInfo.nodes} />
    </div>
  );
};
export default TopScreen;
