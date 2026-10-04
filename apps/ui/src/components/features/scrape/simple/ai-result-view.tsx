import { Sparkles } from 'lucide-react'
import type { ReactNode } from 'react'

import { Alert, AlertDescription, AlertTitle } from '@/components/ui/alert'
import { Badge } from '@/components/ui/badge'
import { JsonBlock } from '@/components/ui/json-block'
import { Spinner } from '@/components/ui/spinner'
import { Stack } from '@/components/ui/stack'
import { Tabs, TabsContent, TabsList, TabsTrigger } from '@/components/ui/tabs'
import { Text } from '@/components/ui/typography'
import { pickRows, type TAiState } from './ai-format'
import { AiRowsTable } from './ai-rows-table'
import { AiDetailView } from './detail/ai-detail-view'
import { isRecord } from './detail/values'

function primaryView(data: unknown): { label: string; content: ReactNode } | null {
  const rows = pickRows(data)
  if (rows) {
    return { label: 'Table', content: <AiRowsTable rows={rows} /> }
  }
  if (isRecord(data)) {
    return { label: 'Details', content: <AiDetailView record={data} /> }
  }
  return null
}

export function AiResultView({ ai }: { ai: TAiState }) {
  if (ai.status === 'pending') {
    return (
      <Stack direction="row" align="center" gap="sm" className="py-8">
        <Spinner />
        <Text variant="muted">Formatting the page with AI…</Text>
      </Stack>
    )
  }
  if (ai.status === 'error') {
    return (
      <Alert variant="destructive">
        <AlertTitle>AI formatting failed</AlertTitle>
        <AlertDescription>{ai.error}. The raw data is still in the other tabs.</AlertDescription>
      </Alert>
    )
  }
  if (ai.status !== 'done') {
    return null
  }
  const primary = primaryView(ai.data)
  const json = <JsonBlock data={ai.data} maxHeight="max-h-[40rem]" />
  return (
    <Stack gap="md">
      <Stack direction="row" align="center" gap="sm">
        <Sparkles className="size-4 text-muted-foreground" />
        <Text variant="muted">Structured by</Text>
        <Badge variant="secondary">{ai.model}</Badge>
      </Stack>
      {primary ? (
        <Tabs defaultValue="primary">
          <TabsList variant="line">
            <TabsTrigger value="primary">{primary.label}</TabsTrigger>
            <TabsTrigger value="json">JSON</TabsTrigger>
          </TabsList>
          <TabsContent value="primary" className="pt-3">
            {primary.content}
          </TabsContent>
          <TabsContent value="json" className="pt-3">
            {json}
          </TabsContent>
        </Tabs>
      ) : (
        json
      )}
    </Stack>
  )
}
