import { useForm } from "react-hook-form"
import { zodResolver } from "@hookform/resolvers/zod"
import { z } from "zod";

type FromType = {
  name: string;
  // organizationType: OrganizationType; // 後々追加
}

const schema = z.object({
  name: z.string().max(50)
})
export const useCreateOrganizationForm = () => {
  const form = useForm<FromType>({ resolver: zodResolver(schema) })
  return { form }
}
