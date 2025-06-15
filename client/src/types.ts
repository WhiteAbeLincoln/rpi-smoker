import { useEffect, useState } from 'react'
import { ZodType, ZodTypeDef, z } from 'zod'

export const SensorDef = z.union([
  z.object({ Dummy: z.object({ file: z.optional(z.string()) }) }),
  z.object({ I2C: z.object({ bus: z.number() }) }),
  z.object({ Gpio: z.object({ pin: z.number() }) }),
])
export type SensorDefT = z.infer<typeof SensorDef>
const SensorDefGeneric = z.record(z.record(z.string()))

export const Config = z.object({
  temp_sensors: z.record(SensorDefGeneric).default(() => ({})),
  fan_location: z.optional(z.string()),
  poll_rate: z.number().gte(0).default(0),
  alarms: z.record(z.string()).default(() => ({})),
  history_size: z.number().gte(0).default(0),
})
export type ConfigT = z.infer<typeof Config>

export const SensorData = z.object({
  ts: z.number().gte(0),
  temps: z.record(z.number()),
})
export type SensorDataT = z.infer<typeof SensorData>

export const TickData = z.object({
  history: z.array(SensorData),
  active_alarms: z.array(z.string()),
})
export type TickDataT = z.infer<typeof TickData>

export const InitialMessage = z.object({
  tick: TickData,
  cfg: Config,
})
export type InitialMessageT = z.infer<typeof InitialMessage>

export const UpdateMessage = z.object({
  hist_upd: z.optional(SensorData),
  active_alarms: z.array(z.string()),
})
export type UpdateMessageT = z.infer<typeof UpdateMessage>

export const Message = z.union([InitialMessage, UpdateMessage])
export type MessageT = z.infer<typeof Message>

export function parseData<O, D extends ZodTypeDef, I>(v: z.ZodType<O, D, I>) {
  return (data: string) => {
    let res: unknown
    try {
      res = JSON.parse(data)
    } catch (e) {
      console.error('failed to parse data', e)
    }

    return v.safeParse(res)
  }
}

export function getStorageValue<T>(
  key: string,
  defaultValue: (() => T) | T,
): T {
  const saved = localStorage.getItem(key)
  if (saved) {
    try {
      return JSON.parse(saved)
    } catch (e) {
      console.error(`Failed to get localstorage ${key}`, e)
    }
  }
  return typeof defaultValue === 'function'
    ? (defaultValue as () => T)()
    : defaultValue
}

export function useLocalStorage<T>(key: string, defaultValue: (() => T) | T) {
  const [value, setValue] = useState(() => getStorageValue(key, defaultValue))
  useEffect(() => {
    localStorage.setItem(key, JSON.stringify(value))
  }, [key, value])

  return [value, setValue] as const
}
