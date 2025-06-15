import { forwardRef, useImperativeHandle } from 'react'
import './FormStyle.css'
import { SchemaInputFormProps, useSchemaInputFormProps } from './useValidation'
import FormControl from './FormControl'
import { pickFormCtrlProps } from './FormControlCtx'
import { FormHelperText } from './FormHelperText'
import { FormLabel } from './FormLabel'
import { JsonSchemaBooleanType } from './JsonSchema'

export type BooleanFormProps = SchemaInputFormProps<JsonSchemaBooleanType>
export const BooleanForm = forwardRef<HTMLInputElement, BooleanFormProps>(
  function BooleanForm(props, ref) {
    const { props: rest, innerRef } = useSchemaInputFormProps(props)
    useImperativeHandle(ref, () => innerRef.current as HTMLInputElement, [
      innerRef,
    ])

    return (
      <FormControl {...pickFormCtrlProps(props)}>
        <FormLabel />
        <input ref={innerRef} type="checkbox" {...rest} />
        <FormHelperText />
      </FormControl>
    )
  },
)

export default BooleanForm
