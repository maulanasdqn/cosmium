import { useState } from 'react'
import { toast } from 'sonner'

import { useScrape } from '@/apis/scrape'
import { Page, PageHeader } from '@/components/ui/page'
import { Grid, Stack } from '@/components/ui/stack'
import { toErrorMessage } from '@/libs/http'
import { recordRun, runFromError, runFromResult, useRun } from '@/stores/history'
import { RerunNotice } from './rerun-notice'
import { ResultPanel } from './result-panel'
import { ScrapeForm } from './scrape-form'
import {
  fromScrapePayload,
  scrapeFormDefaults,
  toScrapePayload,
  type TScrapeFormValues,
} from './schema'

export function ScrapePage({ fromRun, profile }: { fromRun?: string; profile?: string }) {
  const run = useRun(fromRun ?? '')
  const scrape = useScrape()
  const [error, setError] = useState<string | null>(null)
  const defaultValues = run
    ? fromScrapePayload(run.payload)
    : { ...scrapeFormDefaults, profile: profile ?? scrapeFormDefaults.profile }

  async function handleSubmit(values: TScrapeFormValues) {
    const payload = toScrapePayload(values)
    setError(null)
    try {
      const result = await scrape.mutateAsync(payload)
      recordRun(runFromResult(payload, result))
      if (result.blocked) {
        toast.warning('The page looks blocked', { description: result.final_url })
      } else {
        toast.success('Scrape finished', { description: result.final_url })
      }
    } catch (cause) {
      const message = toErrorMessage(cause)
      setError(message)
      recordRun(runFromError(payload, message))
      toast.error('Scrape failed', { description: message })
    }
  }

  return (
    <Page>
      <PageHeader
        title="Scrape"
        description="Load a page through a fingerprint profile and extract what you need."
      />
      {run ? <RerunNotice url={run.payload.url} startedAt={run.startedAt} /> : null}
      <Grid columns={1} gap="lg" className="xl:grid-cols-2">
        <ScrapeForm
          key={run?.id ?? profile ?? 'new'}
          defaultValues={defaultValues}
          pending={scrape.isPending}
          onSubmit={handleSubmit}
        />
        <Stack className="xl:sticky xl:top-20 xl:self-start">
          <ResultPanel
            pending={scrape.isPending}
            result={error ? undefined : scrape.data}
            error={error}
          />
        </Stack>
      </Grid>
    </Page>
  )
}
