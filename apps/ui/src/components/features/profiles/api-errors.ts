import { AxiosError } from 'axios'

export function errorStatus(error: unknown): number | null {
  return error instanceof AxiosError ? (error.response?.status ?? null) : null
}

export function isNotFound(error: unknown): boolean {
  return errorStatus(error) === 404
}

export function isUnreadable(error: unknown): boolean {
  return errorStatus(error) === 422
}
