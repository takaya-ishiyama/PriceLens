import { OrganizationType } from "app/infrastructure/graphql";
import { useForm } from "react-hook-form"

type FromType = {
  name: string;
  // organizationType: OrganizationType; // 後々追加
}

export const useCreateOrganizationForm = () => {
  const form = useForm<FromType>()
  return { form }
}
