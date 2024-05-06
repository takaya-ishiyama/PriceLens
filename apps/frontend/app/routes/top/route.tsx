import { useNavigate } from "@remix-run/react";
import { RouteName } from "app/consts/route_name";
import React from "react";

const TopScreen = () => {
  const navigate = useNavigate();
  const [organizationId, setOrganizationId] = React.useState<string | null>("5cda43e9-abfe-4ddd-800c-c1a8dedb4bcf")

  const onClick = React.useCallback(() => {
    if (organizationId === null) return
    navigate(RouteName.organization(organizationId))
  }, [organizationId, navigate, RouteName.organization])
  return (
    <div>
      <button onClick={onClick} >to organization</button>
    </div>
  )
}
export default TopScreen;
