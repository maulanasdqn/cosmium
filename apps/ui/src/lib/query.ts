import { QueryClient, queryOptions } from "@tanstack/react-query"
import { checkHealth, listProfiles } from "./api"

export const queryClient = new QueryClient({
  defaultOptions: {
    queries: {
      staleTime: 30_000,
      retry: 1,
    },
  },
})

export const queryKeys = {
  health: ["health"] as const,
  profiles: ["profiles"] as const,
}

export const healthQueryOptions = queryOptions({
  queryKey: queryKeys.health,
  queryFn: checkHealth,
  refetchInterval: 15_000,
})

export const profilesQueryOptions = queryOptions({
  queryKey: queryKeys.profiles,
  queryFn: listProfiles,
})
