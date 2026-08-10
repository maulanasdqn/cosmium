import { getApiKey } from "./auth"

export type ScrapeRequest = {
  url: string
  profile: string
  screenshot: boolean
  headful: boolean
  wait_ms: number
  extract: string[]
  script?: string
  proxy?: string
  include_html: boolean
}

export type ScrapeResponse = {
  url: string
  final_url: string
  http_status: number
  html_length: number
  html?: string
  user_agent: string
  cookies_count: number
  blocked: boolean
  extracted: Record<string, unknown>
  screenshot_base64?: string
  elapsed_ms: number
}

export type ProfileListResponse = {
  profiles: string[]
}

export type ErrorResponse = {
  error: string
}

function headers(): Record<string, string> {
  const h: Record<string, string> = { "Content-Type": "application/json" }
  const key = getApiKey()
  if (key) h["x-api-key"] = key
  return h
}

async function call<T>(path: string, init?: RequestInit): Promise<T> {
  const res = await fetch(path, {
    ...init,
    headers: { ...headers(), ...init?.headers },
  })
  const body = await res.json()
  if (!res.ok) {
    throw new Error((body as ErrorResponse).error || `HTTP ${res.status}`)
  }
  return body as T
}

export async function checkHealth(): Promise<boolean> {
  try {
    const res = await fetch("/api/health")
    return res.ok
  } catch {
    return false
  }
}

export async function verifyToken(token: string): Promise<boolean> {
  try {
    const res = await fetch("/api/auth/verify", {
      method: "POST",
      headers: { "x-api-key": token },
    })
    return res.ok
  } catch {
    return false
  }
}

export async function listProfiles(): Promise<string[]> {
  const data = await call<ProfileListResponse>("/api/v1/profiles")
  return data.profiles
}

export async function scrape(req: ScrapeRequest): Promise<ScrapeResponse> {
  return call<ScrapeResponse>("/api/v1/scrape", {
    method: "POST",
    body: JSON.stringify(req),
  })
}

export type GenerateProfileRequest = {
  persona: string
  name: string
}

export type DiagnosticDto = {
  severity: string
  code: string
  message: string
}

export type GenerateProfileResponse = {
  profile: Record<string, unknown>
  diagnostics: DiagnosticDto[]
}

export type SaveProfileRequest = {
  name: string
  profile: Record<string, unknown>
}

export type SaveProfileResponse = {
  saved: string
}

export async function generateProfile(
  req: GenerateProfileRequest,
): Promise<GenerateProfileResponse> {
  return call<GenerateProfileResponse>("/api/v1/profiles/generate", {
    method: "POST",
    body: JSON.stringify(req),
  })
}

export async function saveProfile(
  req: SaveProfileRequest,
): Promise<SaveProfileResponse> {
  return call<SaveProfileResponse>("/api/v1/profiles/save", {
    method: "POST",
    body: JSON.stringify(req),
  })
}
