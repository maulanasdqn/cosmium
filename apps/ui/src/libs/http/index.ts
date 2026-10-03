import axios, { AxiosError } from 'axios'
import { authStore, signOut } from '@/stores/auth'

export type TApiErrorBody = {
  error: string
}

export const apiClient = axios.create({ baseURL: '/api' })

apiClient.interceptors.request.use((config) => {
  const apiKey = authStore.state.apiKey
  if (apiKey) {
    config.headers.set('x-api-key', apiKey)
  }
  return config
})

apiClient.interceptors.response.use(
  (response) => response,
  (error: unknown) => {
    if (error instanceof AxiosError && error.response?.status === 401) {
      signOut()
    }
    return Promise.reject(error)
  },
)

export function toErrorMessage(error: unknown): string {
  if (error instanceof AxiosError) {
    const body = error.response?.data as Partial<TApiErrorBody> | undefined
    return body?.error ?? error.message
  }
  return error instanceof Error ? error.message : String(error)
}
