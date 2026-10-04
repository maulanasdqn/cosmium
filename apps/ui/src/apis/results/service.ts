import { apiClient } from '@/libs/http'
import type {
  TCreateResultPayload,
  TResultList,
  TResultSummary,
  TSetResultAiPayload,
  TStoredResult,
} from './types'

const BASE = '/v1/results'

export const resultsService = {
  list: () => apiClient.get<TResultList>(BASE),
  get: (id: string) => apiClient.get<TStoredResult>(`${BASE}/${encodeURIComponent(id)}`),
  create: (payload: TCreateResultPayload) => apiClient.post<TResultSummary>(BASE, payload),
  setAi: ({ id, ai }: TSetResultAiPayload) =>
    apiClient.put<TResultSummary>(`${BASE}/${encodeURIComponent(id)}/ai`, ai),
  remove: (id: string) =>
    apiClient.delete<{ deleted: string }>(`${BASE}/${encodeURIComponent(id)}`),
}
