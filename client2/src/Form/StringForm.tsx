import React, { forwardRef, useImperativeHandle } from 'react'
import './FormStyle.css'
import { SchemaInputFormProps, useSchemaInputFormProps } from './useValidation'
import FormControl from './FormControl'
import { FormHelperText } from './FormHelperText'
import { FormLabel } from './FormLabel'
import { JsonSchemaStringType } from './JsonSchema'

function getPattern(pattern?: string) {
  if (!pattern) {
    return undefined
  }

  // json-schema (and our validator) treat an unanchored pattern
  // as matching if it matches anywhere in the string
  // html forms treat unanchored patterns as matching the whole string, so
  // we must anchor them ourselves
  if (!pattern.startsWith('^')) {
    pattern = '^(.*)' + pattern
  }
  if (!pattern.endsWith('$')) {
    pattern = pattern + '(.*)$'
  }

  return pattern
}

const inputTypesMap = {
  'date-time': 'datetime-local',
  'time': 'time',
  'date': 'date',
  'email': 'email',
  'uri': 'url',
  'color': 'color',
  'datetime-local': 'datetime-local',
  'month': 'month',
  'number': 'number',
  'password': 'password',
  'search': 'search',
  'tel': 'tel',
  'text': 'text',
  'url': 'url',
  'week': 'week',
} as const satisfies Record<string, React.HTMLInputTypeAttribute>

export type StringFormProps = SchemaInputFormProps<JsonSchemaStringType>
export const StringForm = forwardRef<HTMLInputElement, StringFormProps>(
  function StringForm(props, ref) {
    const { schema, error, label } = props
    const type =
      (
        inputTypesMap as Record<
          string,
          React.HTMLInputTypeAttribute | undefined
        >
      )[schema.format ?? 'text'] ?? 'text'

    const { props: rest, innerRef } = useSchemaInputFormProps(props)
    useImperativeHandle(ref, () => innerRef.current as HTMLInputElement, [
      innerRef,
    ])

    return (
      <FormControl schema={schema} error={error} label={label}>
        <FormLabel />
        <input
          ref={innerRef}
          type={type}
          minLength={schema.minLength}
          maxLength={schema.maxLength}
          pattern={getPattern(schema.pattern)}
          {...rest}
        />
        <FormHelperText />
      </FormControl>
    )
  },
)

export default StringForm
