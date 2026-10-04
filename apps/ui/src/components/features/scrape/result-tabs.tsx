import type { ReactNode } from 'react'
import { Code, FileText, Image, Info } from 'lucide-react'

import type { TScrapeResult } from '@/apis/scrape'
import { Card, CardContent } from '@/components/ui/card'
import { CodeBlock, ImagePreview } from '@/components/ui/code-block'
import { JsonBlock } from '@/components/ui/json-block'
import { Empty, EmptyDescription, EmptyHeader, EmptyTitle } from '@/components/ui/empty'
import { Tabs, TabsContent, TabsList, TabsTrigger } from '@/components/ui/tabs'
import { ResultDetails } from './result-details'

function Nothing({ title, description }: { title: string; description: string }) {
  return (
    <Empty>
      <EmptyHeader>
        <EmptyTitle>{title}</EmptyTitle>
        <EmptyDescription>{description}</EmptyDescription>
      </EmptyHeader>
    </Empty>
  )
}

function Panel({ value, children }: { value: string; children: ReactNode }) {
  return (
    <TabsContent value={value}>
      <Card>
        <CardContent>{children}</CardContent>
      </Card>
    </TabsContent>
  )
}

export function ResultTabs({ result }: { result: TScrapeResult }) {
  const extracted = Object.keys(result.extracted).length > 0
  return (
    <Tabs defaultValue="extracted">
      <TabsList>
        <TabsTrigger value="extracted">
          <Code />
          Extracted
        </TabsTrigger>
        <TabsTrigger value="screenshot">
          <Image />
          Screenshot
        </TabsTrigger>
        {result.html ? (
          <TabsTrigger value="html">
            <FileText />
            HTML
          </TabsTrigger>
        ) : null}
        <TabsTrigger value="details">
          <Info />
          Details
        </TabsTrigger>
      </TabsList>
      <Panel value="extracted">
        {extracted ? (
          <JsonBlock data={result.extracted} />
        ) : (
          <Nothing
            title="Nothing extracted"
            description="Add CSS selectors or a script to pull data out of the page."
          />
        )}
      </Panel>
      <Panel value="screenshot">
        {result.screenshot_base64 ? (
          <ImagePreview
            src={`data:image/jpeg;base64,${result.screenshot_base64}`}
            alt={`Screenshot of ${result.final_url}`}
          />
        ) : (
          <Nothing title="No screenshot" description="Turn on Screenshot to capture the page." />
        )}
      </Panel>
      {result.html ? (
        <Panel value="html">
          <CodeBlock value={result.html} maxHeight="max-h-[32rem]" />
        </Panel>
      ) : null}
      <Panel value="details">
        <ResultDetails result={result} />
      </Panel>
    </Tabs>
  )
}
