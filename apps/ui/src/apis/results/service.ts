import { apiClient } from '@/libs/http'
import type {
  TCreateResultPayload,
  TDeletedCount,
  TResultList,
  TResultStats,
  TResultSummary,
  TSetResultAiPayload,
  TStoredResult,
} from './types'

const BASE = '/v1/results'

export const resultsService = {
  list: () => apiClient.get<TResultList>(BASE),
  stats: () => apiClient.get<TResultStats>(`${BASE}/stats`),
  get: (id: string) => apiClient.get<TStoredResult>(`${BASE}/${encodeURIComponent(id)}`),
  create: (payload: TCreateResultPayload) => apiClient.post<TResultSummary>(BASE, payload),
  setAi: ({ id, ai }: TSetResultAiPayload) =>
    apiClient.put<TResultSummary>(`${BASE}/${encodeURIComponent(id)}/ai`, ai),
  remove: (id: string) =>
    apiClient.delete<{ deleted: string }>(`${BASE}/${encodeURIComponent(id)}`),
  removeMany: (ids: string[]) => apiClient.delete<TDeletedCount>(BASE, { data: { ids } }),
  prune: () => apiClient.post<TDeletedCount>(`${BASE}/prune`),
}
