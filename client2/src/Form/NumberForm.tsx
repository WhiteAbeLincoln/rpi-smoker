import { forwardRef, useImperativeHandle } from 'react'
import './FormStyle.css'
import { SchemaInputFormProps, useSchemaInputFormProps } from './useValidation'
import FormControl from './FormControl'
import { pickFormCtrlProps } from './FormControlCtx'
import { FormHelperText } from './FormHelperText'
import { FormLabel } from './FormLabel'
import { JsonSchemaNumberType } from './JsonSchema'

function roundUp(val: number, mult: number) {
  return mult * Math.ceil(val / mult)
}
function roundDown(val: number, mult: number) {
  return mult * Math.floor(val / mult)
}

function getMax(
  maximum: number | undefined,
  exclusiveMax: number | undefined,
  multipleOf: number | undefined,
) {
  if (maximum === undefined) {
    return undefined
  }
  if (multipleOf === undefined) {
    return maximum
  }
  // return the nearest multiple
  return roundDown(maximum, multipleOf)
}

function getMin(
  minimum: number | undefined,
  exclusiveMin: number | undefined,
  multipleOf: number | undefined,
) {
  if (minimum === undefined) {
    return undefined
  }
  if (multipleOf === undefined) {
    return minimum
  }
  return roundUp(minimum, multipleOf)
}

export type NumberFormProps = SchemaInputFormProps<JsonSchemaNumberType>
export const NumberForm = forwardRef<HTMLInputElement, NumberFormProps>(
  function NumberForm(props, ref) {
    const { schema } = props

    const step =
      (schema.multipleOf as number | undefined) ??
      (schema.type === 'integer' ? 1 : undefined)

    const { props: rest, innerRef } = useSchemaInputFormProps(props)
    useImperativeHandle(ref, () => innerRef.current as HTMLInputElement, [
      innerRef,
    ])

    return (
      <FormControl {...pickFormCtrlProps(props)}>
        <FormLabel />
        <input
          ref={innerRef}
          type="number"
          min={getMin(schema.minimum, schema.exclusiveMinimum, step)}
          max={getMax(schema.maximum, schema.exclusiveMaximum, step)}
          step={step ?? 'any'}
          {...rest}
        />
        <FormHelperText />
      </FormControl>
    )
  },
)

export default NumberForm
