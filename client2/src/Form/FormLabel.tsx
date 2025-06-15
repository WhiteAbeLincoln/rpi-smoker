import { ComponentProps, ElementType, ReactNode } from 'react'
import { useFormControl } from './FormControlCtx'
import clsx, { ClassValue } from 'clsx'

type FormLabelProps<As extends ElementType<{ children: ReactNode }> = 'label'> =
  { as?: As; className?: ClassValue } & ComponentProps<As>
export function FormLabel<As extends ElementType<{ children: ReactNode }>>(
  props: FormLabelProps<As>,
) {
  const { as: comp, children, className, ...rest } = props
  const ctx = useFormControl()
  const Comp = (comp ?? 'label') as ElementType<{ children: ReactNode }>
  const label = children || ctx.label || ctx.schema?.title
  return label ? (
    <Comp
      id={ctx.errorId}
      className={clsx('FormLabel', className, {
        invalid: Boolean(ctx.error),
      })}
      {...rest}
    >
      {label}
    </Comp>
  ) : (
    <></>
  )
}
