import { ComponentProps, ElementType, PropsWithChildren, useId } from 'react'
import clsx, { ClassValue } from 'clsx'
import { BaseFormControlProps, FormControlProvider } from './FormControlCtx'
import './FormStyle.css'
import { JsonSchemaType } from './JsonSchema'

export type FormControlProps<
  T extends JsonSchemaType,
  As extends ElementType = 'div',
> = PropsWithChildren<
  BaseFormControlProps<T> & {
    as?: As
    className?: ClassValue
  } & Omit<ComponentProps<As>, 'children'>
>

export function FormControl<
  T extends JsonSchemaType,
  As extends ElementType = 'div',
>(props: FormControlProps<T, As>) {
  const {
    className,
    as: comp,
    children,
    error,
    label,
    schema,
    id: propId,
    formCtrlId,
    ...rest
  } = props
  const Comp = (comp ?? 'div') as ElementType

  const genId = useId()
  const id = propId || genId
  const errorId = `${id}-err`

  return (
    <FormControlProvider value={{ id, errorId, error, label, schema }}>
      <Comp
        className={clsx(className, 'FormControl')}
        id={formCtrlId}
        {...rest}
      >
        {children}
      </Comp>
    </FormControlProvider>
  )
}

export default FormControl
