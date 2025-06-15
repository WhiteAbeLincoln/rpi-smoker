import { Equal, Or, RequireKey1 } from '../util'

/* eslint-disable @typescript-eslint/ban-types */
export type ReadonlyJsonObject = { readonly [k: string]: ReadonlyJson }
export type ReadonlyJsonArray = readonly ReadonlyJson[]
export type ReadonlyJson =
  | string
  | number
  | boolean
  | null
  | ReadonlyJsonObject
  | ReadonlyJsonArray

export type JsonSchemaBaseProps<T> = {
  $schema?: string
  title?: string
  description?: string
  examples?: Array<
    NonNullable<T extends { readonly default?: infer V } ? V : never>
  >
  readOnly?: boolean
  writeOnly?: boolean
  deprecated?: boolean
  $comment?: string
  default?: ReadonlyJson
  // default?: unknown
  // technically shouldn't allow a def to hold a ref, since that can cause a loop in json-schema's resolver
  // but we don't care to type that since it'll make things complicated
  // $defs?: Record<string, JsonSchemaType>
}
type JsonSchemaBaseProperties<T> = T extends object
  ? T & JsonSchemaBaseProps<T>
  : T

export type JsonSchemaStringType = JsonSchemaBaseProperties<{
  type: 'string'
  minLength?: number
  maxLength?: number
  pattern?: string
  format?: string
  default?: string
}>
export type JsonSchemaNumberType = JsonSchemaBaseProperties<{
  type: 'number' | 'integer'
  multipleOf?: number
  minimum?: number
  exclusiveMinimum?: number
  maximum?: number
  exclusiveMaximum?: number
  default?: number
}>
export type JsonSchemaObjectType = JsonSchemaBaseProperties<{
  type: 'object'
  properties?: Record<string, JsonSchemaType>
  additionalProperties?: JsonSchemaType | false
  unevaluatedProperties?: false
  required?: string[]
  default?: ReadonlyJsonObject
}>
export type JsonSchemaArrayType = JsonSchemaBaseProperties<{
  type: 'array'
  items?: boolean | JsonSchemaType
  prefixItems?: JsonSchemaType[]
  contains?: JsonSchemaType
  minContains?: number
  maxContains?: number
  minItems?: number
  maxItems?: number
  uniqueItems?: number
  default?: ReadonlyJsonArray
}>
export type JsonSchemaBooleanType = JsonSchemaBaseProperties<{
  type: 'boolean'
  default?: boolean
}>
export type JsonSchemaNullType = JsonSchemaBaseProperties<{
  type: 'null'
  default?: null
}>
export type JsonSchemaLiteralType = JsonSchemaBaseProperties<{
  const: ReadonlyJson
  default?: ReadonlyJson
}>
export type JsonSchemaUnknownType = JsonSchemaBaseProperties<{
  default?: ReadonlyJson
}>
export type JsonSchemaAnyOfType = {
  anyOf: JsonSchemaType[] | [JsonSchemaDiscUnionType, ...JsonSchemaType[]]
  default?: ReadonlyJson
}
export type JsonSchemaDiscUnionIfThen = {
  if: JsonSchemaObjectType
  then: JsonSchemaType
}
export type JsonSchemaDiscUnionType = {
  allOf: JsonSchemaDiscUnionIfThen[]
}
export type JsonSchemaRefType = {
  $ref: string
}

export type JsonSchemaType =
  | JsonSchemaStringType
  | JsonSchemaNumberType
  | JsonSchemaObjectType
  // | JsonSchemaArrayType
  | JsonSchemaBooleanType
// | JsonSchemaNullType
// | JsonSchemaLiteralType
// | JsonSchemaUnknownType
// | JsonSchemaAnyOfType
// | JsonSchemaRefType
// | JsonSchemaDiscUnionType

type _JsonSchemaObjProps<
  A extends JsonSchemaObjectType,
  T extends A['properties'] = A['properties'],
  N extends NonNullable<T> = NonNullable<T>,
> =
  Or<
    [Equal<N, never>, Equal<N, NonNullable<JsonSchemaObjectType['properties']>>]
  > extends '1'
    ? unknown
    : { [k in keyof N]?: JsonSchemaToTs<N[k]> }

type _JsonSchemaAdditionalProps<
  A extends JsonSchemaObjectType,
  T extends A['additionalProperties'] = A['additionalProperties'],
  N extends NonNullable<T> = NonNullable<T>,
> =
  Or<
    [
      Equal<N, false>,
      Equal<N, NonNullable<JsonSchemaObjectType['additionalProperties']>>,
    ]
  > extends '1'
    ? unknown
    : {
        [k in string]?: N extends JsonSchemaType
          ? JsonSchemaToTs<N>
          : ReadonlyJson
      }

type JsonSchemaObjToTs<
  T extends JsonSchemaObjectType,
  V = _JsonSchemaObjProps<T> & _JsonSchemaAdditionalProps<T>,
> =
  Equal<V, unknown> extends '1'
    ? Record<PropertyKey, never>
    : RequireKey1<
        V,
        [T['required']] extends [(infer R extends string)[]]
          ? R & keyof V
          : never
      >

export type JsonSchemaToTs<
  T extends JsonSchemaType,
  Type extends T['type'] = T['type'],
> = Type extends 'string'
  ? string
  : Type extends 'number'
    ? number
    : Type extends 'integer'
      ? number
      : Type extends 'boolean'
        ? boolean
        : Type extends 'object'
          ? T extends JsonSchemaObjectType
            ? JsonSchemaObjToTs<T>
            : never
          : never
