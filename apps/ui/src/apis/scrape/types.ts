import type { TWorkflowStep } from '@/components/features/scrape/workflow/types'

export type TProxyRotation = 'round-robin' | 'random'

export type TScrapePayload = {
  url: string
  profile: string
  wait_ms: number
  screenshot: boolean
  headful: boolean
  include_html: boolean
  geo_sync: boolean
  retries: number
  extract: string[]
  script: string | null
  workflow: TWorkflowStep[]
  proxy: string | null
  proxies: string[]
  proxy_rotation: TProxyRotation | null
  wait_for_api: string | null
}

export type TScrapeResult = {
  url: string
  final_url: string
  http_status: number
  html_length: number
  html?: string | null
  user_agent: string
  cookies_count: number
  blocked: boolean
  extracted: Record<string, unknown>
  screenshot_base64?: string | null
  proxy_used?: string | null
  elapsed_ms: number
  attempts: number
}

export type TFormatPayload = {
  url: string
  instruction: string | null
  data: unknown
}

export type TFormatResult = {
  data: unknown
  model: string
}
