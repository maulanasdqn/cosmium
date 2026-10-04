import { apiClient } from '@/libs/http'
import type { TFormatPayload, TFormatResult, TScrapePayload, TScrapeResult } from './types'

export const scrapeService = {
  run: (payload: TScrapePayload) => apiClient.post<TScrapeResult>('/v1/scrape', payload),
  format: (payload: TFormatPayload) => apiClient.post<TFormatResult>('/v1/scrape/format', payload),
}
