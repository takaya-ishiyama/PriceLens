import { Button } from "@/components/ui/button";
import { Link, useLoaderData } from "@remix-run/react";
import { RouteName } from "app/consts/route_name";
import type { FindManyOrganizationWithPageneQuery } from "app/infrastructure/graphql";

type Props = {
  nodes: FindManyOrganizationWithPageneQuery["organizationFindAllWithPagenate"]["nodes"];
};

export const NodeOrganizations: React.FC<Props> = ({ nodes }) => {
  return (
    <div>
      <div>
        {nodes.map((ognz) => (
          <div key={ognz.id} className="flex">
            <Link to={`${RouteName.organization(ognz.id)}`}>
              <div>{ognz.name}</div>
            </Link>
          </div>
        ))}
      </div>
      <Button onClick={() => { }}>
        <div>{"次のページへ"}</div>
      </Button>
    </div>
  );
};
