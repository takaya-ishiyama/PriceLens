import type React from "react";
import { type InputProps, Input as SCNInput } from "@/components/ui/input";

type Props = {} & InputProps;
export const Input: React.FC<Props> = ({ ...props }) => {
  return (
    <SCNInput {...props} />
  )
}
