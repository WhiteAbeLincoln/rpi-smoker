import { ComponentProps, ElementType, ReactNode } from 'react'
import { useFormControl } from './FormControlCtx'
import clsx, { ClassValue } from 'clsx'

type FormHelperTextProps<
  As extends ElementType<{ children: ReactNode }> = 'p',
> = { as?: As; className?: ClassValue } & ComponentProps<As>
export function FormHelperText<As extends ElementType<{ children: ReactNode }>>(
  props: FormHelperTextProps<As>,
) {
  const { as: comp, children, className, ...rest } = props
  const ctx = useFormControl()
  const Comp = (comp ?? 'p') as ElementType<{ children: ReactNode }>
  return (
    <Comp
      id={ctx.errorId}
      className={clsx('FormHelperText', className, {
        invalid: Boolean(ctx.error),
      })}
      {...rest}
    >
      {children || ctx.error || ctx.schema?.description || ' '}
    </Comp>
  )
}
