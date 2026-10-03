import { apiClient } from '@/libs/http'
import type { THealth } from './types'

export const healthService = {
  get: () => apiClient.get<THealth>('/health'),
}
