import { useQuery } from '@tanstack/react-query'
import { healthService } from './service'

export const healthKeys = {
  all: ['health'] as const,
}

export function useHealth() {
  return useQuery({
    queryKey: healthKeys.all,
    queryFn: () => healthService.get().then((res) => res.data),
    refetchInterval: 15_000,
  })
}
