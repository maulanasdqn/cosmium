import { useMutation } from '@tanstack/react-query'
import { testsService } from './service'
import type { TFingerprintPayload, TStealthPayload } from './types'

export function useFingerprintTest() {
  return useMutation({
    mutationFn: (payload: TFingerprintPayload) =>
      testsService.fingerprint(payload).then((res) => res.data),
  })
}

export function useStealthTest() {
  return useMutation({
    mutationFn: (payload: TStealthPayload) => testsService.stealth(payload).then((res) => res.data),
  })
}
