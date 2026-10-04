import { useMutation } from '@tanstack/react-query'
import { scrapeService } from './service'
import type { TFormatPayload, TScrapePayload } from './types'

export function useScrape() {
  return useMutation({
    mutationFn: (payload: TScrapePayload) => scrapeService.run(payload).then((res) => res.data),
  })
}

export function useFormatScrape() {
  return useMutation({
    mutationFn: (payload: TFormatPayload) => scrapeService.format(payload).then((res) => res.data),
  })
}
