import { useState } from 'react'

import { useFormatScrape, type TScrapeResult } from '@/apis/scrape'
import { toErrorMessage } from '@/libs/http'
import type { TAiState } from './ai-format'
import { readPageData, readSelectorMatches } from './page-data'

export type TAiOutput = {
  data: unknown
  model: string
}

export function useAiFormat(initial: TAiState = { status: 'off' }) {
  const format = useFormatScrape()
  const [ai, setAi] = useState<TAiState>(initial)

  async function run(result: TScrapeResult, instruction: string): Promise<TAiOutput | null> {
    setAi({ status: 'pending' })
    try {
      const response = await format.mutateAsync({
        url: result.final_url,
        instruction: instruction.trim() || null,
        data: { page: readPageData(result), selectors: readSelectorMatches(result) },
      })
      setAi({ status: 'done', data: response.data, model: response.model })
      return response
    } catch (cause) {
      setAi({ status: 'error', error: toErrorMessage(cause) })
      return null
    }
  }

  return { ai, run }
}
