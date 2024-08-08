import { Button } from "@/components/ui/button";
import { useLoaderData } from "@remix-run/react";
import type { FindManyOrganizationWithPageneQuery } from "app/infrastructure/graphql";

export const NodeOrganizations: React.FC = () => {
  const { nodes, pageInfo } =
    useLoaderData<
      FindManyOrganizationWithPageneQuery["organizationFindAllWithPagenate"]
    >();

  return (
    <div>
      <div>
        {nodes.map((ognz) => (
          <div key={ognz.id}>{ognz.name}</div>
        ))}
      </div>
      <Button>
        <div>{"次のページへ"}</div>
      </Button>
    </div>
  );
};
