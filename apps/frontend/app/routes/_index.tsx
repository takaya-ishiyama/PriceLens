import { Button } from "@/components/ui/button";
import type { MetaFunction } from "@remix-run/node";
import { Link, useNavigate } from "@remix-run/react";
import { RouteName } from "app/consts/route_name";

export const meta: MetaFunction = () => {
  return [{ title: "book recommend" }];
};

export default function Index() {
  const navigate = useNavigate();
  const handleClickNavigateToTop = () => navigate(RouteName.top);
  return (
    <div>
      <div className={"grid place-content-center"}>
        <Button onClick={handleClickNavigateToTop}>get top page</Button>
      </div>
    </div>
  );
}
