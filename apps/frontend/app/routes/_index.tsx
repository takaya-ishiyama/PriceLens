import { Button } from "@/components/ui/button";
import type { MetaFunction } from "@remix-run/node";
import { Link } from "@remix-run/react";
import { RouteName } from "app/consts/route_name";

export const meta: MetaFunction = () => {
  return [{ title: "book recommend" }];
};

export default function Index() {
  return (
    <div>
      <div className={"grid place-content-center"}>
        <Link to={RouteName.top}>get top page</Link>
      </div>
    </div>
  )
}
