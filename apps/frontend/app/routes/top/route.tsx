import { useNavigate } from "@remix-run/react";
import { RouteName } from "app/consts/route_name";
import React from "react";
import { useCreateOrganization } from "./hooks/useCreateOrganization";
import { Button } from "@/components/ui/button";
import { InputWithRHF } from "@/components/molecule/Input/InputWithRHF";
import { useCreateOrganizationForm } from "./hooks/useCreateOrganizationForm";

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

  const onClickGoToOrganization = React.useCallback(() => {
    if (organizationId === null) return;
    navigate(RouteName.organization(organizationId));
  }, [organizationId, navigate, RouteName.organization]);

  return (
    <div>
      <div>
        <Button onClick={onClickGoToOrganization}>to organization</Button>
      </div>
      <div>
        <button onClick={handleSubmit}>create organizatioon</button>
        {/* modalにする予定 */}
        <InputWithRHF name={"name"} control={control} />
      </div>
    </div>
  );
};
export default TopScreen;
