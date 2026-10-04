import { useEffect, useRef } from 'react'
import { toast } from 'sonner'

import { useHealth } from '@/apis/health'
import { useSetResultAi, type TStoredResult } from '@/apis/results'
import { toErrorMessage } from '@/libs/http'
import type { TAiState } from '../simple/ai-format'
import { useAiFormat } from '../simple/use-ai-format'

function initialState(stored: TStoredResult): TAiState {
  return stored.ai
    ? { status: 'done', data: stored.ai.data, model: stored.ai.model }
    : { status: 'off' }
}

export function useSavedAi(stored: TStoredResult) {
  const health = useHealth()
  const save = useSetResultAi()
  const { ai, run } = useAiFormat(initialState(stored))
  const started = useRef(false)
  const available = health.data?.llm_configured ?? false

  async function format(instruction: string) {
    const output = await run(stored.result, instruction)
    if (!output) {
      return
    }
    try {
      await save.mutateAsync({ id: stored.id, ai: { ...output, instruction: instruction || null } })
    } catch (cause) {
      toast.error('Could not save the AI result', { description: toErrorMessage(cause) })
    }
  }

  useEffect(() => {
    const request = stored.ai_request
    if (started.current || stored.ai || !request?.enabled || !available) {
      return
    }
    started.current = true
    void format(request.instruction)
  })

  return {
    ai,
    available,
    canFormat: available && ai.status !== 'pending',
    format: () => format(stored.ai_request?.instruction ?? ''),
  }
}
