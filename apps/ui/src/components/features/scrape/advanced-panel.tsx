import type { TScrapePayload } from '@/apis/scrape'
import { Alert, AlertDescription, AlertTitle } from '@/components/ui/alert'
import { Stack } from '@/components/ui/stack'
import type { TRunEntry } from '@/stores/history'
import { RerunNotice } from './rerun-notice'
import { ScrapeForm } from './scrape-form'
import { fromScrapePayload, scrapeFormDefaults, toScrapePayload } from './schema'

export function AdvancedPanel({
  run,
  profile,
  pending,
  error,
  onSubmit,
}: {
  run: TRunEntry | null
  profile?: string
  pending: boolean
  error: string | null
  onSubmit: (payload: TScrapePayload) => Promise<unknown>
}) {
  const defaultValues = run
    ? fromScrapePayload(run.payload)
    : { ...scrapeFormDefaults, profile: profile ?? scrapeFormDefaults.profile }

  return (
    <Stack gap="lg">
      {run ? <RerunNotice url={run.payload.url} startedAt={run.startedAt} /> : null}
      {error ? (
        <Alert variant="destructive">
          <AlertTitle>The scrape failed</AlertTitle>
          <AlertDescription>{error}</AlertDescription>
        </Alert>
      ) : null}
      <ScrapeForm
        key={run?.id ?? profile ?? 'new'}
        defaultValues={defaultValues}
        pending={pending}
        onSubmit={async (values) => {
          await onSubmit(toScrapePayload(values))
        }}
      />
    </Stack>
  )
}
