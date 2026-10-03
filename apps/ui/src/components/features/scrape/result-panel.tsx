import type { TScrapeResult } from '@/apis/scrape'
import { Stack } from '@/components/ui/stack'
import { ResultError, ResultIdle, ResultPending } from './result-states'
import { ResultSummary } from './result-summary'
import { ResultTabs } from './result-tabs'

export function ResultPanel({
  pending,
  result,
  error,
}: {
  pending: boolean
  result: TScrapeResult | undefined
  error: string | null
}) {
  if (pending) {
    return <ResultPending />
  }
  if (error) {
    return <ResultError message={error} />
  }
  if (!result) {
    return <ResultIdle />
  }
  return (
    <Stack gap="md">
      <ResultSummary result={result} />
      <ResultTabs result={result} />
    </Stack>
  )
}
