import { useMutation } from '@tanstack/react-query'
import { scrapeService } from './service'
import type { TScrapePayload } from './types'

export function useScrape() {
  return useMutation({
    mutationFn: (payload: TScrapePayload) => scrapeService.run(payload).then((res) => res.data),
  })
}
