import type { TScrapePayload, TScrapeResult } from '@/apis/scrape'

export type TAiRequest = {
  enabled: boolean
  instruction: string
}

export type TStoredAi = {
  data: unknown
  model: string
  instruction: string | null
}

export type TResultSummary = {
  id: string
  created_at_ms: number
  title: string | null
  url: string | null
  final_url: string | null
  profile: string | null
  http_status: number | null
  blocked: boolean
  elapsed_ms: number | null
  has_ai: boolean
}

export type TStoredResult = {
  id: string
  created_at_ms: number
  title: string | null
  payload: TScrapePayload
  result: TScrapeResult
  ai_request: TAiRequest | null
  ai: TStoredAi | null
}

export type TCreateResultPayload = {
  title: string | null
  payload: TScrapePayload
  result: TScrapeResult
  ai_request: TAiRequest | null
}

export type TResultList = {
  results: TResultSummary[]
}

export type TSetResultAiPayload = {
  id: string
  ai: TStoredAi
}
