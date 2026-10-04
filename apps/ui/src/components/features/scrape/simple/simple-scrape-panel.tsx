import { useHealth } from '@/apis/health'
import { useProfileSummaries } from '@/apis/profiles'
import type { TAiRequest } from '@/apis/results'
import type { TScrapePayload } from '@/apis/scrape'
import { Alert, AlertDescription, AlertTitle } from '@/components/ui/alert'
import { Skeleton } from '@/components/ui/skeleton'
import { Stack } from '@/components/ui/stack'
import { useScrapePrefs } from './scrape-prefs-store'
import { SimpleScrapeForm } from './simple-scrape-form'
import { SimplePending } from './simple-pending'

export function SimpleScrapePanel({
  initialProfile,
  pending,
  error,
  onSubmit,
}: {
  initialProfile?: string
  pending: boolean
  error: string | null
  onSubmit: (payload: TScrapePayload, ai: TAiRequest) => Promise<void>
}) {
  const summaries = useProfileSummaries()
  const health = useHealth()
  const prefs = useScrapePrefs()

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

  return (
    <Stack gap="lg">
      <SimpleScrapeForm
        key={profile}
        profiles={profiles}
        prefs={{ ...prefs, profile }}
        pending={pending}
        aiAvailable={health.data?.llm_configured ?? false}
        onSubmit={onSubmit}
      />
      {pending ? <SimplePending /> : null}
      {!pending && error ? (
        <Alert variant="destructive">
          <AlertTitle>The scrape failed</AlertTitle>
          <AlertDescription>{error}</AlertDescription>
        </Alert>
      ) : null}
    </Stack>
  )
}
