import { Input } from "@/components/atom";
import type React from "react";
import { Controller, type FieldValues, type UseControllerProps } from "react-hook-form";

type InputProps = React.ComponentProps<typeof Input>;
type ErrorProps = {}; //FIXME: 直す

type OmitForInputWithRHF = Omit<InputProps, "value" | "onChange" | "onBlur">;

type InputWithRHFProps<
  TFieldValues extends FieldValues,
  Props = Record<never, never>,
> = UseControllerProps<TFieldValues> & {
  inputProps?: OmitForInputWithRHF;
  errorProps?: ErrorProps;
  children?: React.ReactNode;
} & Props;

export const InputWithRHF = <TFieldValues extends FieldValues>({
  inputProps,
  errorProps,
  children,
  ...rhfProps
}: InputWithRHFProps<TFieldValues>) => {
  return (
    <Controller
      {...rhfProps}
      render={({
        field: { onChange, onBlur, value, ref },
        fieldState: { error },
      }) => (
        <div>
          <div>
            <Input
              {...inputProps}
              value={value ?? ""}
              onChange={onChange}
              onBlur={onBlur}
            />
            {children}
          </div>
          <div>{error?.message != null ? [error.message] : []}</div>
        </div>
      )}
    />
  );
};
