import { useState } from 'react'
import JsonForm from './Form/JsonForm'

export default function ComponentForm({ schema }: { schema: object }) {
  const [state, setState] = useState({ input: 1 } as unknown as {
    input: string
    error?: string
  })
  // const onValidate: ValidateFn<string> = data => {
  //   console.log('validate', data)
  //   setState(data)
  // }
  return (
    <JsonForm
      schema={{
        type: 'object',
        properties: {
          x: { type: 'number' },
        },
        additionalProperties: false,
        title: 'Digits',
        description: 'test',
      }}
      error={state.error}
    />
  )
}
