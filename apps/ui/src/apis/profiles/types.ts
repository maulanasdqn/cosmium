export type TSeverity = 'error' | 'warning' | 'info'

export type TDiagnostic = {
  severity: TSeverity
  field: string
  message: string
}

export type TProfile = Record<string, unknown>

export type TProfileSummary = {
  name: string
  navigator_platform?: string | null
  client_hints_platform?: string | null
  chrome_version?: string | null
  user_agent?: string | null
  gpu_renderer?: string | null
  timezone?: string | null
  languages?: string[] | null
  screen?: string | null
  device_pixel_ratio?: number | null
  errors?: number | null
  warnings?: number | null
  load_error?: string | null
}

export type TProfileSummaries = {
  profiles: TProfileSummary[]
}

export type TDiagnostics = {
  diagnostics: TDiagnostic[]
}

export type TProfileWithDiagnostics = {
  profile: TProfile
  diagnostics: TDiagnostic[]
}

export type TGenerateProfilePayload = {
  persona: string
  name: string
}

export type TSaveProfilePayload = {
  name: string
  profile: TProfile
}

export type TSaveProfileResponse = {
  saved: string
}

export type TDeleteProfileResponse = {
  deleted: string
}

export type TMutateProfilePayload = {
  name: string
  count: number
  hint: string | null
}

export type TMutateProfileResponse = {
  variants: TProfileWithDiagnostics[]
}
