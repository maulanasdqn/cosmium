import { useMutation, useQuery, useQueryClient } from '@tanstack/react-query'
import { resultsService } from './service'
import type { TCreateResultPayload, TSetResultAiPayload } from './types'

export const resultKeys = {
  all: ['results'] as const,
  lists: () => [...resultKeys.all, 'list'] as const,
  details: () => [...resultKeys.all, 'detail'] as const,
  detail: (id: string) => [...resultKeys.details(), id] as const,
}

export function useResults() {
  return useQuery({
    queryKey: resultKeys.lists(),
    queryFn: () => resultsService.list().then((res) => res.data.results),
  })
}

export function useResult(id: string) {
  return useQuery({
    queryKey: resultKeys.detail(id),
    queryFn: () => resultsService.get(id).then((res) => res.data),
    staleTime: Number.POSITIVE_INFINITY,
    retry: false,
  })
}

export function useCreateResult() {
  const queryClient = useQueryClient()
  return useMutation({
    mutationFn: (payload: TCreateResultPayload) =>
      resultsService.create(payload).then((res) => res.data),
    onSuccess: () => queryClient.invalidateQueries({ queryKey: resultKeys.lists() }),
  })
}

export function useSetResultAi() {
  const queryClient = useQueryClient()
  return useMutation({
    mutationFn: (payload: TSetResultAiPayload) =>
      resultsService.setAi(payload).then((res) => res.data),
    onSuccess: (_, { id }) =>
      Promise.all([
        queryClient.invalidateQueries({ queryKey: resultKeys.lists() }),
        queryClient.invalidateQueries({ queryKey: resultKeys.detail(id) }),
      ]),
  })
}

export function useDeleteResult() {
  const queryClient = useQueryClient()
  return useMutation({
    mutationFn: (id: string) => resultsService.remove(id).then((res) => res.data),
    onSuccess: () => queryClient.invalidateQueries({ queryKey: resultKeys.all }),
  })
}
