import { Page, PageHeader } from '@/components/ui/page'
import { Tabs, TabsContent, TabsList, TabsTrigger } from '@/components/ui/tabs'
import { useRun } from '@/stores/history'
import { AdvancedPanel } from './advanced-panel'
import { SimpleScrapePanel } from './simple/simple-scrape-panel'
import { useRunScrape } from './use-run-scrape'

export function ScrapePage({ fromRun, profile }: { fromRun?: string; profile?: string }) {
  const run = useRun(fromRun ?? '')
  const scrape = useRunScrape()

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
          <SimpleScrapePanel
            initialProfile={profile}
            pending={scrape.pending}
            error={scrape.error}
            onSubmit={scrape.run}
          />
        </TabsContent>
        <TabsContent value="advanced" className="pt-4">
          <AdvancedPanel
            run={run}
            profile={profile}
            pending={scrape.pending}
            error={scrape.error}
            onSubmit={(payload) => scrape.run(payload)}
          />
        </TabsContent>
      </Tabs>
    </Page>
  )
}
