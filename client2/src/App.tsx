import './App.css'
import FlowEditor, { BuiltInComp } from './FlowEditor'

const BuiltInComponents: BuiltInComp[] = [
  {
    id: '1',
    label: 'Run On Schedule',
    type: 'source',
    inputForm: {
      type: 'object',
      properties: {
        schedule: {
          type: 'string',
          description: 'The cron schedule',
        },
      },
      required: ['schedule'],
    },
    outputForm: {
      type: 'null',
    },
  },
  {
    id: '2',
    label: 'Save Datapoint',
    type: 'sink',
    inputForm: {
      type: 'object',
      properties: {
        data: 'string',
        metadata: 'object',
      },
      required: ['data'],
    },
    outputForm: {
      type: 'null',
    },
  },
  {
    id: '3',
    label: 'RPI I2C ADS 1115',
    type: 'pipe',
    inputForm: {
      type: 'object',
      properties: {
        gain: { enum: ['two-thirds', 1, 2, 4, 8, 16], default: 2 },
        channel: {
          enum: [
            'Single A0',
            'Single A1',
            'Single A2',
            'Single A3',
            'Differential A0 A1',
            'Differential A0 A3',
            'Differential A1 A3',
            'Differential A2 A3',
          ],
          default: 'Single A0',
        },
        bus: { type: 'integer', minimum: 0, maximum: 255 },
        address: { type: 'integer', minimum: 0, maximum: 65535 },
      },
      required: ['gain', 'channel'],
      additionalProperties: false,
    },
    outputForm: {
      type: 'object',
      properties: {
        raw: 'integer',
        voltage: 'number',
      },
      required: ['raw', 'voltage'],
    },
  },
  {
    id: '4',
    label: 'Calculate',
    type: 'pipe',
    inputForm: {
      type: 'object',
      properties: {
        expression: { type: 'string' },
        inputs: { type: 'array', items: { type: 'string' } },
      },
      required: ['expression', 'inputs'],
      additionalProperties: false,
    },
    // inputForm: {
    //   type: 'object',
    //   properties: {
    //     expression: 'string',
    //     inputs: 'string',
    //   },
    //   required: ['expression', 'inputs'],
    //   additionalProperties: false,
    // },
    outputForm: {
      type: 'object',
      properties: {
        value: {},
      },
      required: ['value'],
      additionalProperties: false,
    },
  },
  {
    id: '5',
    label: 'Watch Datapoint',
    type: 'source',
    inputForm: {
      type: 'object',
      properties: {
        filter_expression: 'string',
      },
      required: [],
      additionalProperties: false,
    },
    outputForm: {
      type: 'object',
      properties: {
        value: 'number',
        metadata: {},
      },
      required: ['value', 'metadata'],
      additionalProperties: false,
    },
  },
]
export default function App() {
  return (
    <>
      <nav>
        <button>Dashboard</button>
        <button>Flow Editor</button>
      </nav>
      <FlowEditor
        style={{ flex: '1 1 auto', gap: '2rem' }}
        availableNodes={BuiltInComponents}
      />
    </>
  )
}
