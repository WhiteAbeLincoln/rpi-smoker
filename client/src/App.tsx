import React, { ReactNode, useEffect, useRef, useState } from 'react'
import './App.css'
import {
  Brush,
  CartesianGrid,
  Legend,
  Line,
  LineChart,
  ResponsiveContainer,
  Tooltip,
  XAxis,
  YAxis,
} from 'recharts'
import { Config, ConfigT, Message, SensorDataT, parseData } from './types'
import {
  Button,
  Container,
  Divider,
  Form,
  Icon,
  List,
  Tab,
} from 'semantic-ui-react'
import { ConfigForm, useCfg } from './Configuration'

function useWebsocket(hostname: string, port: string) {
  const [isReady, setIsReady] = useState(false)
  const [data, setData] = useState<unknown>()
  const ws = useRef<WebSocket>()

  useEffect(() => {
    let socket: WebSocket | undefined
    if (port) {
      socket = new WebSocket(`ws://${hostname}:${port}/ws`)
      socket.addEventListener('open', () => {
        setIsReady(true)
      })
      socket.addEventListener('close', () => {
        setIsReady(false)
      })
      socket.addEventListener('message', event => {
        setData(event.data)
      })

      ws.current = socket
    }
    return () => {
      socket?.close()
    }
  }, [hostname, port])

  return {
    isReady,
    data,
    send(data: string | ArrayBufferLike | Blob | ArrayBufferView) {
      ws.current?.send(data)
    },
    close() {
      ws.current?.close()
    },
  }
}

type SensorDataPoint = Omit<SensorDataT, 'ts'> & {
  ts: Date
}

function mkDataPoint(d: SensorDataT): SensorDataPoint {
  return { ...d, ts: new Date(d.ts) }
}

type State = {
  alarms: Record<string, boolean>
  sensorData: SensorDataPoint[]
}

function truncate_history(history: SensorDataPoint[], hist_size?: number) {
  const last = history[history.length - 1] as SensorDataPoint | undefined
  if (!last || hist_size === undefined) {
    return
  }

  // duration in milliseconds
  const duration = hist_size * 1000
  let ts = last.ts.valueOf() - duration
  let found_idx: number | undefined = undefined
  history.some((val, idx) => {
    if (val.ts.valueOf() >= ts) {
      found_idx = idx
      return true
    }
    return false
  })

  history.slice(found_idx ?? history.length)
}

function Alarms({
  data,
  onAck,
}: {
  data: Record<string, boolean>
  onAck?: (data: [string, boolean]) => void
}) {
  return (
    <>
      <h2>Alarms</h2>
      <List horizontal>
        {Object.entries(data).map(([k, ack]) => (
          <List.Item key={k} style={{ color: ack ? 'green' : 'red' }}>
            <List.Content>
              <Button
                icon
                labelPosition="left"
                onClick={() => onAck?.([k, !ack])}
              >
                <Icon
                  name={!ack ? 'times' : 'check'}
                  color={!ack ? 'red' : 'green'}
                />
                {ack ? `UnAck ${k}` : `Ack ${k}`}
              </Button>
            </List.Content>
          </List.Item>
        ))}
      </List>
    </>
  )
}

function convertNum(v: string): number | undefined {
  if (!v) {
    return
  }
  const num = Number(v)
  if (Number.isNaN(num)) {
    return
  }
  return num
}

function getNum(
  input: FanConfigPointInput,
  key: keyof FanConfigPointInput,
): Either<number, string> {
  const num = convertNum(input[key])
  console.log('converting num', input[key], num)
  if (num === undefined) {
    return { error: 'Not a Number' }
  }
  return { value: num }
}
function getDuty(input: FanConfigPointInput): Either<number, string> {
  const num = getNum(input, 'dutyCycle')
  if (num.error !== undefined) {
    return num
  }
  if (num.value < 0 || num.value > 100) {
    return { error: 'Not a percentage' }
  }
  return num
}
function getFanCfgPoint(
  text: FanConfigPointInput,
): Either<FanConfigPoint, string> {
  const dutyCycle = getDuty(text)
  if (dutyCycle.error !== undefined) {
    return dutyCycle
  }
  const temp = getNum(text, 'temp')
  if (temp.error !== undefined) {
    return temp
  }
  return { value: { dutyCycle: dutyCycle.value, temp: temp.value } }
}

type Either<A, B> =
  | { value: A; error?: undefined }
  | { value?: undefined; error: B }
type FanConfigPoint = { dutyCycle: number; temp: number }
type FanConfigPointInput = { [k in keyof FanConfigPoint]: string }
type FanConfigParsed = { input: FanConfigPointInput; value?: FanConfigPoint }
function FanConfigForm({
  input,
  onChange,
  children,
}: {
  input: FanConfigPoint | FanConfigPointInput
  onChange?: (v: FanConfigParsed) => void
  children?: ReactNode
}) {
  const pointInput = Object.fromEntries(
    Object.entries(input).map(([k, v]) => [k, String(v)]),
  ) as FanConfigPointInput
  function doChange(
    k: keyof FanConfigPointInput,
    v: FanConfigPointInput[keyof FanConfigPointInput],
  ) {
    const next = { ...pointInput, [k]: v }
    const value = getFanCfgPoint(next).value
    onChange?.({
      input: next,
      value,
    })
  }

  return (
    <Form.Group widths="equal">
      <Form.Input
        fluid
        label="Duty Cycle"
        type="number"
        value={pointInput.dutyCycle}
        onChange={e => doChange('dutyCycle', e.target.value)}
        error={getDuty(pointInput).error}
        min={0}
        max={100}
      />
      <Form.Input
        fluid
        label="Temp"
        type="number"
        value={pointInput.temp}
        onChange={e => doChange('temp', e.target.value)}
        error={getNum(pointInput, 'temp').error}
      />
      {children}
    </Form.Group>
  )
}

type FanConfigData = FanConfigPoint[]
function FanConfig() {
  const [state, setState] = useState([] as FanConfigData)
  const [newItem, setNewItem] = useState<FanConfigParsed>({
    input: {
      dutyCycle: '',
      temp: '',
    },
  })

  function addItem(s: FanConfigPoint) {
    setState(old => {
      const newState = old.filter(o => o.temp !== s.temp)
      newState.push(s)
      return newState.sort((a, b) => a.temp - b.temp)
    })
    setNewItem(() => ({
      input: {
        dutyCycle: '',
        temp: '',
      },
    }))
  }
  function removeItem(idx: number) {
    setState(old => {
      const next = [...old]
      next.splice(idx, 1)
      return next
    })
  }

  return (
    <Container>
      <h2>Fan Config</h2>
      <ResponsiveContainer width="100%" height={600}>
        <LineChart data={state} onClick={e => console.log('click', e)}>
          <CartesianGrid strokeDasharray="3 3" />
          <Line dot={true} dataKey="dutyCycle" />
          <XAxis
            dataKey="temp"
            type="number"
            domain={[0, Math.max(300, ...state.map(s => s.temp))]}
            label="Temperature"
          />
          <YAxis domain={[0, 100]} label="Duty Cycle" />
          <Tooltip allowEscapeViewBox={{ x: true, y: true }} />
          <Legend />
        </LineChart>
      </ResponsiveContainer>
      <Form>
        <FanConfigForm input={newItem.input} onChange={setNewItem}>
          <Button
            icon
            onClick={() => newItem.value && addItem(newItem.value)}
            disabled={!newItem.value}
          >
            <Icon name="plus" />
          </Button>
        </FanConfigForm>
        <Divider />
        {state.map((d, i) => (
          <FanConfigForm input={d}>
            <Button icon onClick={() => removeItem(i)}>
              <Icon name="trash" />
            </Button>
          </FanConfigForm>
        ))}
      </Form>
      <pre>{JSON.stringify(newItem.value, null, 2)}</pre>
    </Container>
  )
}

function Graph({
  sensorData,
  cfg,
}: {
  sensorData: State['sensorData']
  cfg?: ConfigT
}) {
  const [brushVal, setBrush] = useState(
    {} as { startIndex?: number; endIndex?: number },
  )
  // const prevData = usePrevious(sensorData)
  // useEffect(() => {
  //   setBrush(b => {
  //     if (b.endIndex === prevData?.length) {
  //       return { ...b, endIndex: undefined }
  //     }
  //     return b
  //   })
  // }, [sensorData, prevData])
  return (
    <Container>
      <h2>History</h2>
      <ResponsiveContainer width="100%" height={600}>
        <LineChart data={sensorData}>
          <CartesianGrid strokeDasharray="3 3" />
          {Object.keys(cfg?.temp_sensors ?? {}).map(k => (
            <Line dot={false} name={k} dataKey={d => d.temps[k]} />
          ))}
          <XAxis
            dataKey="ts"
            angle={-45}
            tickFormatter={(v: Date) => v.toLocaleTimeString()}
            mirror={true}
            tickMargin={35}
            dx={25}
          />
          <Brush
            dataKey="ts"
            height={30}
            data={sensorData}
            {...brushVal}
            onChange={setBrush}
          />
          <YAxis />
          <Tooltip allowEscapeViewBox={{ x: true, y: true }} />
          <Legend />
        </LineChart>
      </ResponsiveContainer>
    </Container>
  )
}

function App() {
  const [state, setState] = useState<State>({
    alarms: {},
    sensorData: [],
  })
  const { cfg, setCfg } = useCfg()
  const { data: newData } = useWebsocket(cfg.local.hostname, cfg.local.port)

  useEffect(() => {
    if (typeof newData !== 'string') {
      return
    }
    const result = parseData(Message)(newData)
    if (!result.success) {
      console.error('Failed to parse message', result.error)
      return
    }
    console.log('got new data', result)
    const r = result.data
    if ('tick' in r) {
      setCfg({ server: r.cfg })
    }

    setState(oldState => {
      const newState = { ...oldState }

      if ('tick' in r) {
        newState.sensorData = r.tick.history.map(mkDataPoint)
        truncate_history(newState.sensorData, r.cfg.history_size)
        newState.alarms = addAlarms(oldState.alarms, r.tick.active_alarms)
      } else {
        if (r.hist_upd) {
          newState.sensorData = [
            ...structuredClone(newState.sensorData),
            mkDataPoint(r.hist_upd),
          ]
          truncate_history(newState.sensorData, cfg.server?.history_size)
        }
        newState.alarms = addAlarms(oldState.alarms, r.active_alarms)
      }

      return newState
    })
  }, [newData])

  const tabPanes = [
    {
      menuItem: 'Dashboard',
      render: () => (
        <Tab.Pane className="TabPane">
          <Alarms
            data={state.alarms}
            onAck={([k, v]) =>
              setState(o => ({ ...o, alarms: { ...o.alarms, [k]: v } }))
            }
          />
          <Graph sensorData={state.sensorData} cfg={cfg.server} />
        </Tab.Pane>
      ),
    },
    {
      menuItem: 'Fan Config',
      render: () => (
        <Tab.Pane className="TabPane">
          <FanConfig />
        </Tab.Pane>
      ),
    },
    {
      menuItem: 'Config',
      render: () => (
        <Tab.Pane className="TabPane">
          <ConfigForm />
        </Tab.Pane>
      ),
    },
  ]

  return (
    <div className="App">
      <Tab panes={tabPanes} className="TabPane" />
    </div>
  )
}

export default App

function addAlarms(oldAlarms: Readonly<State['alarms']>, alarms: string[]) {
  const newAlarms: Record<string, boolean> = {}
  for (const x of alarms) {
    newAlarms[x] = oldAlarms[x] || false
  }
  return newAlarms
}
