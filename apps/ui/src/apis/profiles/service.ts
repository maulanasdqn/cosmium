import { apiClient } from '@/libs/http'
import type {
  TDeleteProfileResponse,
  TDiagnostics,
  TGenerateProfilePayload,
  TMutateProfilePayload,
  TMutateProfileResponse,
  TProfile,
  TProfileSummaries,
  TProfileWithDiagnostics,
  TSaveProfilePayload,
  TSaveProfileResponse,
} from './types'

const BASE = '/v1/profiles'

const named = (name: string) => `${BASE}/${encodeURIComponent(name)}`

export const profilesService = {
  summaries: () => apiClient.get<TProfileSummaries>(`${BASE}/summaries`),
  get: (name: string) => apiClient.get<TProfile>(named(name)),
  validate: (name: string) => apiClient.get<TDiagnostics>(`${named(name)}/validate`),
  validateDraft: (profile: TProfile) =>
    apiClient.post<TDiagnostics>(`${BASE}/validate`, { profile }),
  remove: (name: string) => apiClient.delete<TDeleteProfileResponse>(named(name)),
  save: (payload: TSaveProfilePayload) =>
    apiClient.post<TSaveProfileResponse>(`${BASE}/save`, payload),
  generate: (payload: TGenerateProfilePayload) =>
    apiClient.post<TProfileWithDiagnostics>(`${BASE}/generate`, payload),
  repair: (name: string) => apiClient.post<TProfileWithDiagnostics>(`${named(name)}/repair`),
  mutate: ({ name, ...body }: TMutateProfilePayload) =>
    apiClient.post<TMutateProfileResponse>(`${named(name)}/mutate`, body),
}
