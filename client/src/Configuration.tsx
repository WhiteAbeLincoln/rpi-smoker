import { Button, Form, Input, Message, TextArea } from 'semantic-ui-react'
import {
  Config,
  ConfigT,
  getStorageValue,
  parseData,
  useLocalStorage,
} from './types'
import { ReactNode, createContext, useContext, useState } from 'react'
const parser = parseData(Config)

export type LocalCfg = {
  hostname: string
  port: string
}
export type Cfg = {
  local: LocalCfg
  server: ConfigT | undefined
}
export const getLocalConfig = () =>
  getStorageValue('config', () => ({
    hostname: window.location.hostname,
    port: window.location.port,
  }))

export type CfgContext = {
  cfg: Cfg
  setCfg: (data: Partial<Cfg>, send?: boolean) => void
}

const CfgCtx = createContext<CfgContext>({
  cfg: {
    local: getLocalConfig(),
    server: undefined,
  },
  setCfg: () => {
    /* noop */
  },
})
export const useCfg = () => useContext(CfgCtx)

export function ConfigForm() {
  const { cfg, setCfg } = useCfg()
  const [localCfg, setLocalCfg] = useState(cfg.local)

  const [text, setText] = useState(JSON.stringify(cfg.server, null, 2))
  const [err, setErr] = useState('')
  function doSubmit(text: string) {
    const val = parser(text)
    if (!val.success) {
      setErr(val.error.flatten().formErrors.join('\n'))
      return
    }
    setErr('')
    setCfg({ server: val.data }, true)
  }
  function doLocalSubmit() {
    setCfg({ local: localCfg })
  }
  return (
    <div
      style={{ display: 'flex', flexFlow: 'column', flex: '1', width: '100%' }}
    >
      <Form style={{ display: 'flex', flexFlow: 'column', flex: '1' }}>
        <Input
          type="text"
          label="Host"
          value={localCfg.hostname}
          onChange={e => setLocalCfg(c => ({ ...c, hostname: e.target.value }))}
        />
        <Input
          type="number"
          min="0"
          max="65535"
          label="Port"
          value={localCfg.port}
          onChange={e => setLocalCfg(c => ({ ...c, port: e.target.value }))}
        />
        <Button type="submit" onClick={() => doLocalSubmit()}>
          Save Client Cfg
        </Button>
      </Form>
      <Form style={{ display: 'flex', flexFlow: 'column', flex: '1' }}>
        <TextArea
          value={text}
          onChange={v => setText(v.target.value)}
          style={{ flex: '1' }}
        />
        <Button type="submit" onClick={() => doSubmit(text)}>
          Save Server Cfg
        </Button>
      </Form>
      {err ? (
        <Message attached="bottom" error>
          {err}
        </Message>
      ) : null}
    </div>
  )
}

export function ConfigProvider({ children }: { children: ReactNode }) {
  const [local, setLocal] = useLocalStorage('config', getLocalConfig())
  const [server, setServer] = useState<ConfigT>()

  const setCfg: CfgContext['setCfg'] = (data, send) => {
    if (data.local) {
      setLocal(data.local)
    }
    if (data.server) {
      setServer(data.server)
      if (send) {
        console.log('TODO: send this to the server')
      }
    }
  }
  return (
    <CfgCtx.Provider
      value={{
        cfg: {
          local,
          server,
        },
        setCfg,
      }}
    >
      {children}
    </CfgCtx.Provider>
  )
}
