import { AxiosError } from 'axios'

import { toErrorMessage } from '@/libs/http'

export function loginErrorMessage(error: unknown): string {
  if (error instanceof AxiosError && error.response?.status === 401) {
    return 'Invalid API key'
  }
  return toErrorMessage(error)
}
