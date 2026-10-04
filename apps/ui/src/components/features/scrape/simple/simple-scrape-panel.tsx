import { useHealth } from '@/apis/health'
import { useProfileSummaries } from '@/apis/profiles'
import type { TScrapePayload, TScrapeResult } from '@/apis/scrape'
import { Alert, AlertDescription, AlertTitle } from '@/components/ui/alert'
import { Skeleton } from '@/components/ui/skeleton'
import { Stack } from '@/components/ui/stack'
import { readPageData, readSelectorMatches } from './page-data'
import { ResultHero } from './result-hero'
import { ResultSections } from './result-sections'
import { useScrapePrefs } from './scrape-prefs-store'
import { SimpleScrapeForm, type TAiRequest } from './simple-scrape-form'
import { SimplePending } from './simple-pending'
import { useAiFormat } from './use-ai-format'

export function SimpleScrapePanel({
  initialProfile,
  pending,
  result,
  error,
  onSubmit,
}: {
  initialProfile?: string
  pending: boolean
  result?: TScrapeResult
  error: string | null
  onSubmit: (payload: TScrapePayload) => Promise<TScrapeResult | null>
}) {
  const summaries = useProfileSummaries()
  const health = useHealth()
  const prefs = useScrapePrefs()
  const { ai, run, reset } = useAiFormat()

  if (summaries.isPending) {
    return <Skeleton className="h-40 w-full" />
  }
  const profiles = (summaries.data ?? [])
    .filter((profile) => !profile.load_error)
    .map((profile) => ({ value: profile.name, label: profile.name }))
  const known = (name?: string) => profiles.some((option) => option.value === name)
  const profile = known(initialProfile)
    ? (initialProfile ?? '')
    : known(prefs.profile)
      ? prefs.profile
      : (profiles[0]?.value ?? '')

  async function handleSubmit(payload: TScrapePayload, request: TAiRequest) {
    reset()
    const scraped = await onSubmit(payload)
    if (scraped && request.enabled) {
      await run(scraped, request.instruction)
    }
  }

  return (
    <Stack gap="lg">
      <SimpleScrapeForm
        key={profile}
        profiles={profiles}
        prefs={{ ...prefs, profile }}
        pending={pending || ai.status === 'pending'}
        aiAvailable={health.data?.llm_configured ?? false}
        onSubmit={handleSubmit}
      />
      {pending ? <SimplePending /> : null}
      {!pending && error ? (
        <Alert variant="destructive">
          <AlertTitle>The scrape failed</AlertTitle>
          <AlertDescription>{error}</AlertDescription>
        </Alert>
      ) : null}
      {!pending && !error && result ? (
        <Stack gap="lg">
          <ResultHero
            result={result}
            data={readPageData(result)}
            formatted={ai.status === 'done' ? ai.data : undefined}
          />
          <ResultSections
            data={readPageData(result)}
            matches={readSelectorMatches(result)}
            ai={ai}
          />
        </Stack>
      ) : null}
    </Stack>
  )
}
