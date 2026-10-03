import { apiClient } from '@/libs/http'
import type { TScrapePayload, TScrapeResult } from './types'

export const scrapeService = {
  run: (payload: TScrapePayload) => apiClient.post<TScrapeResult>('/v1/scrape', payload),
}
