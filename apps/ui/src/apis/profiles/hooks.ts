import { useMutation, useQuery, useQueryClient } from '@tanstack/react-query'
import { profilesService } from './service'
import type { TProfile } from './types'

export const profileKeys = {
  all: ['profiles'] as const,
  summaries: () => [...profileKeys.all, 'summaries'] as const,
  details: () => [...profileKeys.all, 'detail'] as const,
  detail: (name: string) => [...profileKeys.details(), name] as const,
  validation: (name: string) => [...profileKeys.detail(name), 'validation'] as const,
}

export function useProfileSummaries() {
  return useQuery({
    queryKey: profileKeys.summaries(),
    queryFn: () => profilesService.summaries().then((res) => res.data.profiles),
  })
}

export function useProfile(name: string) {
  return useQuery({
    queryKey: profileKeys.detail(name),
    queryFn: () => profilesService.get(name).then((res) => res.data),
  })
}

export function useProfileValidation(name: string) {
  return useQuery({
    queryKey: profileKeys.validation(name),
    queryFn: () => profilesService.validate(name).then((res) => res.data.diagnostics),
  })
}

export function useValidateDraft() {
  return useMutation({
    mutationFn: (profile: TProfile) =>
      profilesService.validateDraft(profile).then((res) => res.data.diagnostics),
  })
}

export function useSaveProfile() {
  const queryClient = useQueryClient()
  return useMutation({
    mutationFn: profilesService.save,
    onSuccess: () => queryClient.invalidateQueries({ queryKey: profileKeys.all }),
  })
}

export function useDeleteProfile() {
  const queryClient = useQueryClient()
  return useMutation({
    mutationFn: profilesService.remove,
    onSuccess: () => queryClient.invalidateQueries({ queryKey: profileKeys.all }),
  })
}

export function useGenerateProfile() {
  return useMutation({
    mutationFn: (payload: Parameters<typeof profilesService.generate>[0]) =>
      profilesService.generate(payload).then((res) => res.data),
  })
}

export function useRepairProfile() {
  return useMutation({
    mutationFn: (name: string) => profilesService.repair(name).then((res) => res.data),
  })
}

export function useMutateProfile() {
  return useMutation({
    mutationFn: (payload: Parameters<typeof profilesService.mutate>[0]) =>
      profilesService.mutate(payload).then((res) => res.data.variants),
  })
}
