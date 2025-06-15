import { ComponentType, ElementType, lazy } from 'react'
import {
  JsonSchemaBooleanType,
  JsonSchemaNumberType,
  JsonSchemaObjectType,
  JsonSchemaStringType,
  JsonSchemaType,
} from './JsonSchema'
import type { BooleanFormProps } from './BooleanForm'
import type { NumberFormProps } from './NumberForm'
import type { ObjectFormProps } from './ObjectForm'
import type { StringFormProps } from './StringForm'
import { Equal, UnionToIntersection } from '../util'

const StringForm = lazy(() => import('./StringForm'))
const NumberForm = lazy(() => import('./NumberForm'))
const BooleanForm = lazy(() => import('./BooleanForm'))
const ObjectForm = lazy(() => import('./ObjectForm'))

type OmitFrom<T, K extends PropertyKey> = T extends unknown ? Omit<T, K> : never

export type JsonFormProps<Schema extends JsonSchemaType> =
  Equal<Schema, JsonSchemaType> extends '1'
    ? UnionToIntersection<
        OmitFrom<
          | StringFormProps
          | NumberFormProps
          | BooleanFormProps
          | ObjectFormProps,
          'schema'
        >
      > & { schema: JsonSchemaType }
    : Schema extends JsonSchemaStringType
      ? StringFormProps
      : Schema extends JsonSchemaNumberType
        ? NumberFormProps
        : Schema extends JsonSchemaBooleanType
          ? BooleanFormProps
          : Schema extends JsonSchemaObjectType
            ? ObjectFormProps<Schema>
            : never

const ObjMap = {
  string: StringForm,
  number: NumberForm,
  integer: NumberForm,
  boolean: BooleanForm,
  object: ObjectForm,
} as const satisfies Record<JsonSchemaType['type'], ElementType>

export function JsonForm<Schema extends JsonSchemaType>(
  props: JsonFormProps<Schema>,
) {
  const Comp = ObjMap[props.schema.type] as ComponentType<typeof props>
  return <Comp {...props} />
}

export default JsonForm
