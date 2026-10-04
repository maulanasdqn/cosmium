import { useState } from 'react'

import { useFormatScrape, type TScrapeResult } from '@/apis/scrape'
import { toErrorMessage } from '@/libs/http'
import type { TAiState } from './ai-format'
import { readPageData, readSelectorMatches } from './page-data'

export function useAiFormat() {
  const format = useFormatScrape()
  const [ai, setAi] = useState<TAiState>({ status: 'off' })

  async function run(result: TScrapeResult, instruction: string) {
    setAi({ status: 'pending' })
    try {
      const response = await format.mutateAsync({
        url: result.final_url,
        instruction: instruction.trim() || null,
        data: { page: readPageData(result), selectors: readSelectorMatches(result) },
      })
      setAi({ status: 'done', data: response.data, model: response.model })
    } catch (cause) {
      setAi({ status: 'error', error: toErrorMessage(cause) })
    }
  }

  return { ai, run, reset: () => setAi({ status: 'off' }) }
}
