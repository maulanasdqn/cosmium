import { useMutation } from '@tanstack/react-query'
import { authService } from './service'

export function useVerifyKey() {
  return useMutation({
    mutationFn: authService.verify,
  })
}
