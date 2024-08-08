import { Button } from "@/components/ui/button";
import { Link, useLoaderData } from "@remix-run/react";
import { RouteName } from "app/consts/route_name";
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
          <Link key={ognz.id} to={`${RouteName.organization(ognz.id)}`}>
            <div >{ognz.name}</div>
          </Link>
        ))}
      </div>
      <Button>
        <div>{"次のページへ"}</div>
      </Button>
    </div>
  );
};
