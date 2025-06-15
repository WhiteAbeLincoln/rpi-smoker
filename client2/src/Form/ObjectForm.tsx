import { ReadonlyJsonObject, ValidateFn } from './useValidation'
import FormControl, { FormControlProps } from './FormControl'
import { FormLabel } from './FormLabel'
import { pickFormCtrlProps } from './FormControlCtx'
import { FormHelperText } from './FormHelperText'
import { JsonSchemaObjectType, JsonSchemaToTs } from './JsonSchema'
import { JsonForm } from './JsonForm'

export type ObjectFormProps<
  T extends JsonSchemaObjectType = JsonSchemaObjectType,
> = {
  value?: JsonSchemaToTs<T>
  onValidate?: ValidateFn<JsonSchemaToTs<T>>
} & Omit<FormControlProps<T>, 'ref'>
export function ObjectForm<T extends JsonSchemaObjectType>(
  props: ObjectFormProps<T>,
) {
  const { schema, value, onValidate } = props
  const schemaProperties = schema.properties

  // const additionalProperties: JSONSchemaType<unknown> | false | undefined =
  //   schema.additionalProperties

  return (
    <FormControl as="fieldset" {...pickFormCtrlProps(props)}>
      <FormLabel as="legend" />
      <FormHelperText />
      {Object.entries(schemaProperties ?? {}).map(([k, v]) => (
        <JsonForm key={k} label={k} schema={v} />
      ))}
    </FormControl>
  )
}

export default ObjectForm
