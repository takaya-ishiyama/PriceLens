import { useNavigate } from "@remix-run/react";
import { RouteName } from "app/consts/route_name";
import React from "react";

const TopScreen = () => {
  const navigate = useNavigate();
  const [organizationId] = React.useState<string | null>(
    "5cda43e9-abfe-4ddd-800c-c1a8dedb4bcf",
  );

  const onClickGoToOrganization = React.useCallback(() => {
    if (organizationId === null) return;
    navigate(RouteName.organization(organizationId));
  }, [organizationId, navigate]);

  return (
    <div>
      <div>
        <button onClick={onClickGoToOrganization}>to organization</button>
      </div>
      <div>
        <button onClick={() => { }}>create organizatioon</button>
      </div>
      {/* <button onClick={onClick}>to organization</button> */}
    </div>
  );
};
export default TopScreen;
