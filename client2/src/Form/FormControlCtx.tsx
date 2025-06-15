import { createContext, useContext } from 'react'
import { omit1, pick1 } from '../util'
import { JsonSchemaType } from './JsonSchema'

export type FormControlContext = {
  errorId: string
  id: string
  error?: string
  label?: string
  schema?: JsonSchemaType
}

export type BaseFormControlProps<T extends JsonSchemaType = JsonSchemaType> = {
  error?: string
  label?: string
  schema: T
  id?: string
  formCtrlId?: string
}
export const pickFormCtrlProps = pick1<keyof BaseFormControlProps>([
  'error',
  'label',
  'schema',
  'id',
  'formCtrlId',
])
export const omitFormCtrlProps = omit1<keyof BaseFormControlProps>([
  'error',
  'label',
  'schema',
  'id',
  'formCtrlId',
])

// eslint-disable-next-line react-refresh/only-export-components
const FormControlCtx = createContext<FormControlContext>({
  id: '',
  errorId: '',
})
export const { Provider: FormControlProvider, Consumer: FormControlConsumer } =
  FormControlCtx
export const useFormControl = () => useContext(FormControlCtx)
