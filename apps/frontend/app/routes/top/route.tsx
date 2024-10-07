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
import { getEnvironment } from "app/infrastructure/dotenv";
// import { getEnvironment } from "app/infrastructure/dotenv";

type GetOrganizationsProps = {
  uri: string
  after?: string;
  before?: string;
};

const getOrganizations = async ({ uri, after, before }: GetOrganizationsProps) => {
  try {
    if ((after !== undefined) && (before !== undefined))
      throw new Error("afterとbeforeは同時に指定できません");
    const requestOptions: RequestOptions<
      FindManyOrganizationWithPageneQueryVariables,
      FindManyOrganizationWithPageneQuery
    > = {
      document: FindManyOrganizationWithPageneDocument,
      variables: { first: 10, after },
    };

    const { organizationFindAllWithPagenate } =
      await client(uri).request(requestOptions);
    return organizationFindAllWithPagenate;
  } catch (e) {
    console.log("エラーはっせい！！", (e as Error).message);
    throw new Error((e as Error).message);
  }
};

export const loader: LoaderFunction = async () => {
  const { BACKEND_URI } = getEnvironment()
  return getOrganizations({ uri: BACKEND_URI });
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

  const [ognzInfo, setOgnzInfo] = React.useState<
    FindManyOrganizationWithPageneQuery["organizationFindAllWithPagenate"]
  >({ nodes, pageInfo });
  const [after, setAfter] = React.useState<string | undefined>(nodes.at(-1)?.id);
  const [before, setBefore] = React.useState<string | undefined>(nodes[0]?.id);


  // const { BACKEND_URI } = getEnvironment()//FIXME: actionでやるhttps://localhost:8080/graphql
  const BACKEND_URI = "http://localhost:8080/graphql"

  // // 初回読み込み
  // React.useEffect(() => {
  //   getOrganizations({})
  //     .then((res) => {
  //       setOgnzInfo(res);
  //       setAfter(res.nodes.at(-1)?.id);
  //     })
  // }, []);

  const handleClickGetNextOgnz = useCallback(async () => {
    if (!ognzInfo.pageInfo.hasNextPage) return;
    const ognz = await getOrganizations({ after, uri: BACKEND_URI });
    setOgnzInfo(ognz);
    if (ognz.nodes[-1]?.id)
      // nodeの最後のidを取得
      setAfter(ognz.nodes[-1]?.id);
    if (ognz.nodes[0]?.id)
      // nodeの最初のidを取得
      setBefore(ognz.nodes[0]?.id);
  }, [after, ognzInfo.pageInfo.hasNextPage]);

  const handleClickGetPrevOgnz = useCallback(async () => {
    if (!ognzInfo.pageInfo.hasPreviousPage) return;
    const ognz = await getOrganizations({ before, uri: BACKEND_URI });
    setOgnzInfo(ognz);
    if (ognz.nodes.at(-1)?.id)
      // nodeの最後のidを取得
      setAfter(ognz.nodes.at(-1)?.id);
    if (ognz.nodes[0]?.id)
      // nodeの最初のidを取得
      setBefore(ognz.nodes[0]?.id);
  }, [before, ognzInfo.pageInfo.hasPreviousPage]);

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
      <NodeOrganizations
        onClickGotoNextPage={handleClickGetNextOgnz}
        onClickGotoPrevPage={handleClickGetPrevOgnz}
        nodes={ognzInfo.nodes}
      />
    </div>
  );
};
export default TopScreen;
