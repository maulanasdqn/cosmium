import { useState } from 'react'
import { toast } from 'sonner'

import { useScrape, type TScrapePayload, type TScrapeResult } from '@/apis/scrape'
import { Page, PageHeader } from '@/components/ui/page'
import { Tabs, TabsContent, TabsList, TabsTrigger } from '@/components/ui/tabs'
import { toErrorMessage } from '@/libs/http'
import { recordRun, runFromError, runFromResult, useRun } from '@/stores/history'
import { AdvancedPanel } from './advanced-panel'
import { SimpleScrapePanel } from './simple/simple-scrape-panel'

export function ScrapePage({ fromRun, profile }: { fromRun?: string; profile?: string }) {
  const run = useRun(fromRun ?? '')
  const scrape = useScrape()
  const [error, setError] = useState<string | null>(null)

  async function runScrape(payload: TScrapePayload): Promise<TScrapeResult | null> {
    setError(null)
    try {
      const result = await scrape.mutateAsync(payload)
      recordRun(runFromResult(payload, result))
      if (result.blocked) {
        toast.warning('The page looks blocked', { description: result.final_url })
      } else {
        toast.success('Scrape finished', { description: result.final_url })
      }
      return result
    } catch (cause) {
      const message = toErrorMessage(cause)
      setError(message)
      recordRun(runFromError(payload, message))
      toast.error('Scrape failed', { description: message })
      return null
    }
  }

  const shared = {
    pending: scrape.isPending,
    result: error ? undefined : scrape.data,
    error,
    onSubmit: runScrape,
  }

  return (
    <Page>
      <PageHeader
        title="Scrape"
        description="Paste a link and choose what to collect. Cosmium opens it in a stealth browser."
      />
      <Tabs defaultValue={run ? 'advanced' : 'simple'}>
        <TabsList>
          <TabsTrigger value="simple">Simple</TabsTrigger>
          <TabsTrigger value="advanced">Advanced</TabsTrigger>
        </TabsList>
        <TabsContent value="simple" className="pt-4">
          <SimpleScrapePanel initialProfile={profile} {...shared} />
        </TabsContent>
        <TabsContent value="advanced" className="pt-4">
          <AdvancedPanel run={run} profile={profile} {...shared} />
        </TabsContent>
      </Tabs>
    </Page>
  )
}
