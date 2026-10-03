import { apiClient } from '@/libs/http'
import type { TVerifyKeyPayload } from './types'

export const authService = {
  verify: ({ apiKey }: TVerifyKeyPayload) =>
    apiClient.post<void>('/auth/verify', null, { headers: { 'x-api-key': apiKey } }),
}
