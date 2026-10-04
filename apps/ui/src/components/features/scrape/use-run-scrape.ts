import { useNavigate } from '@tanstack/react-router'
import { toast } from 'sonner'

import { useCreateResult, type TAiRequest } from '@/apis/results'
import { useScrape, type TScrapePayload } from '@/apis/scrape'
import { toErrorMessage } from '@/libs/http'
import { recordRun, runFromError, runFromResult } from '@/stores/history'
import { readPageData } from './simple/page-data'

export function useRunScrape() {
  const scrape = useScrape()
  const save = useCreateResult()
  const navigate = useNavigate()

  async function run(payload: TScrapePayload, aiRequest: TAiRequest | null = null) {
    try {
      const result = await scrape.mutateAsync(payload)
      const saved = await save.mutateAsync({
        title: readPageData(result).summary?.title || null,
        payload,
        result,
        ai_request: result.blocked ? null : aiRequest,
      })
      recordRun(runFromResult(payload, result, saved.id))
      if (result.blocked) {
        toast.warning('The page looks blocked', { description: result.final_url })
      }
      await navigate({ to: '/scrape/$id', params: { id: saved.id } })
    } catch (cause) {
      const message = toErrorMessage(cause)
      recordRun(runFromError(payload, message))
      toast.error('Scrape failed', { description: message })
    }
  }

  return {
    run,
    pending: scrape.isPending || save.isPending,
    error: scrape.error
      ? toErrorMessage(scrape.error)
      : save.error
        ? toErrorMessage(save.error)
        : null,
  }
}
