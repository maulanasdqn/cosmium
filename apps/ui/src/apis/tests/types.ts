export type TVerdict = 'pass' | 'warn' | 'fail'

export type TProbe = {
  id: string
  passed: boolean
  got: string
  expected: string
  error: string | null
}

export type TFingerprintPayload = {
  profile: string
  geo_sync: boolean
}

export type TFingerprintResult = {
  probes: TProbe[]
  passed: number
  failed: number
}

export type TStealthPayload = {
  profile: string
  bot_check_url: string | null
  geo_sync: boolean
}

export type TStealthCheck = {
  target: string
  verdict: TVerdict
  detail: string
  duration_ms: number
}

export type TStealthResult = {
  results: TStealthCheck[]
}
