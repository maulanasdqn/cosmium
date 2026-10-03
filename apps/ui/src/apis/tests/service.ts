import { apiClient } from '@/libs/http'
import type {
  TFingerprintPayload,
  TFingerprintResult,
  TStealthPayload,
  TStealthResult,
} from './types'

const BASE = '/v1/tests'

export const testsService = {
  fingerprint: (payload: TFingerprintPayload) =>
    apiClient.post<TFingerprintResult>(`${BASE}/fingerprint`, payload),
  stealth: (payload: TStealthPayload) => apiClient.post<TStealthResult>(`${BASE}/stealth`, payload),
}
