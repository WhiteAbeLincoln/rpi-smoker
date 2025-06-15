import clsx from 'clsx'
import React, { FormEvent, InvalidEvent, useEffect, useRef } from 'react'
import { FormControlProps } from './FormControl'
import { omitFormCtrlProps } from './FormControlCtx'
import type {
  JsonSchemaBooleanType,
  JsonSchemaNumberType,
  JsonSchemaStringType,
  JsonSchemaToTs,
  ReadonlyJson,
} from './JsonSchema'

type ErrorState = {
  error: string
  validityState: ValidityState
}
export type ValidateFn<
  I extends ReadonlyJson = ReadonlyJson,
  V extends ReadonlyJson = ReadonlyJson,
> = (
  data:
    | {
        input: I
        value: V
      }
    | ({ input: I } & ErrorState),
) => void

export function useValidation({
  value,
  input,
  onValidate,
}: {
  value: string | number | readonly string[] | undefined
  input: HTMLInputElement | null
  onValidate?: ValidateFn<string>
}) {
  const lastChecked = useRef<string | number | readonly string[]>()

  useEffect(() => {
    if (value !== undefined && value !== lastChecked.current) {
      input?.checkValidity()
    }
  }, [value, lastChecked, input])

  function onInput(event: FormEvent<HTMLInputElement>) {
    const target = event.target as HTMLInputElement
    console.log('got change', event, target.value)
    const valid = Boolean(target.checkValidity())
    if (valid) {
      onValidate?.({
        input: target.value,
        value:
          target.type === 'number'
            ? target.valueAsNumber
            : target.type === 'checkbox'
              ? target.checked
              : target.value,
      })
    }
  }

  function onInvalid(event: InvalidEvent<HTMLInputElement>) {
    console.log('Got on invalid', {
      event,
      evVal: event.target.value,
      value,
      lastChecked,
    })
    lastChecked.current = event.target.value
    const validityState: ValidityState = event.target.validity
    const error = event.target.validationMessage

    onValidate?.({
      input: event.target.value,
      error,
      validityState,
    })
  }

  return { onInput, onInvalid }
}

export type SchemaInputFormProps<
  Schema extends
    | JsonSchemaStringType
    | JsonSchemaNumberType
    | JsonSchemaBooleanType,
> = {
  value?: JsonSchemaToTs<Schema>
  onValidate?: ValidateFn<string>
} & Omit<React.InputHTMLAttributes<HTMLInputElement>, 'className'> &
  Omit<FormControlProps<Schema>, 'ref'>

export function useSchemaInputFormProps<
  Schema extends
    | JsonSchemaStringType
    | JsonSchemaNumberType
    | JsonSchemaBooleanType,
>(propsIn: SchemaInputFormProps<Schema> & { errorId?: string }) {
  const { schema, onValidate, error, className, errorId, ...props } = propsIn
  const innerRef = useRef<HTMLInputElement>(null)
  const { onInput, onInvalid } = useValidation({
    value: props.value,
    input: innerRef.current,
    onValidate: onValidate,
  })

  return {
    props: {
      ...omitFormCtrlProps(props),
      'id': props.id,
      'className': clsx('FormInput', className, { invalid: Boolean(error) }),
      'title': props.title ?? schema.title,
      'aria-errormessage': error ? errorId : undefined,
      'aria-invalid': error ? ('true' as const) : undefined,
      'onInput': (e: FormEvent<HTMLInputElement>) => (
        onInput(e), props.onInput?.(e)
      ),
      'onInvalid': (e: InvalidEvent<HTMLInputElement>) => (
        onInvalid(e), props.onInvalid?.(e)
      ),
    },
    innerRef,
  }
}
