export function omit<O extends object, K extends string | number | symbol>(
  obj: O,
  keys: K[],
): Omit<O, K> {
  return Object.entries(obj).reduce(
    (acc, [key, val]) => {
      if (!keys.includes(key as K)) {
        // eslint-disable-next-line no-extra-semi
        ;(acc as Record<PropertyKey, unknown>)[key] = val
      }
      return acc
    },
    {} as Omit<O, K>,
  )
}

export const omit1 =
  <K extends string | number | symbol>(keys: K[]) =>
  <O extends object>(o: O): Omit<O, K> =>
    omit(o, keys)

export function pick<O extends object, K extends keyof O>(
  obj: O,
  keys: K[],
): Pick<O, K> {
  return keys.reduce(
    (acc, key) => {
      if (Object.prototype.hasOwnProperty.call(obj, key)) {
        acc[key] = obj[key]
      }
      return acc
    },
    {} as Pick<O, K>,
  )
}

export const pick1 =
  <K extends string | number | symbol>(keys: K[]) =>
  <O extends { [k in K]?: unknown }>(o: O): Pick<O, K> =>
    pick(o, keys)

export type SB = '1' | '0'
export type And<A extends readonly SB[]> = Matches<A[number], '1'>
export type Matches<A, B> = [A] extends [B] ? '1' : '0'
export type Equal<A, B> = And<[Matches<A, B>, Matches<B, A>]>
export type Not<B extends SB> = B extends '1' ? '0' : '1'
export type Or<A extends readonly SB[]> = '1' extends A[number] ? '1' : '0'
export type If<Pred extends SB, True, False> = Pred extends '1' ? True : False
export type ValidateMatch<T extends B, B> = T

export type KeyMap<T> = { [k in keyof T]: k }

// see ts-essentials/ts-essentials
// works because in a KeyMap, the only way a property can have undefined for a value is if
// the original property was optional.
export type OptionalKeys<T, M extends KeyMap<T> = KeyMap<T>> = T extends unknown
  ? {
      [K in keyof T]-?: undefined extends M[K] ? K : never
    }[keyof T]
  : never

export type RequireKey<T, K extends OptionalKeys<T>> =
  Equal<K, never> extends '1'
    ? T
    : Omit<T, K> & {
        [k in K]: T[k]
      }

export type RequireKey1<T, K extends keyof T> =
  Equal<K, never> extends '1'
    ? T
    : Omit<T, K> & {
        [k in K]: T[k]
      }

export type UnionToIntersection<U> =
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  (U extends any ? (k: U) => void : never) extends (k: infer I) => void
    ? I
    : never
