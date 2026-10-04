import type { TScrapePayload, TScrapeResult } from '@/apis/scrape'
import { Grid, Stack } from '@/components/ui/stack'
import type { TRunEntry } from '@/stores/history'
import { RerunNotice } from './rerun-notice'
import { ResultPanel } from './result-panel'
import { ScrapeForm } from './scrape-form'
import { fromScrapePayload, scrapeFormDefaults, toScrapePayload } from './schema'

export function AdvancedPanel({
  run,
  profile,
  pending,
  result,
  error,
  onSubmit,
}: {
  run: TRunEntry | null
  profile?: string
  pending: boolean
  result?: TScrapeResult
  error: string | null
  onSubmit: (payload: TScrapePayload) => Promise<unknown>
}) {
  const defaultValues = run
    ? fromScrapePayload(run.payload)
    : { ...scrapeFormDefaults, profile: profile ?? scrapeFormDefaults.profile }

  return (
    <Stack gap="lg">
      {run ? <RerunNotice url={run.payload.url} startedAt={run.startedAt} /> : null}
      <Grid columns={1} gap="lg" className="xl:grid-cols-2">
        <ScrapeForm
          key={run?.id ?? profile ?? 'new'}
          defaultValues={defaultValues}
          pending={pending}
          onSubmit={async (values) => {
            await onSubmit(toScrapePayload(values))
          }}
        />
        <Stack className="xl:sticky xl:top-20 xl:self-start">
          <ResultPanel pending={pending} result={error ? undefined : result} error={error} />
        </Stack>
      </Grid>
    </Stack>
  )
}
