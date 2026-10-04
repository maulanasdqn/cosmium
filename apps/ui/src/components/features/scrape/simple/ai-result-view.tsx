import { Sparkles } from 'lucide-react'

import { Alert, AlertDescription, AlertTitle } from '@/components/ui/alert'
import { Badge } from '@/components/ui/badge'
import { CodeBlock } from '@/components/ui/code-block'
import { Spinner } from '@/components/ui/spinner'
import { Stack } from '@/components/ui/stack'
import { Text } from '@/components/ui/typography'
import { pickRows, type TAiState } from './ai-format'
import { AiRowsTable } from './ai-rows-table'

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
  const rows = pickRows(ai.data)
  return (
    <Stack gap="md">
      <Stack direction="row" align="center" gap="sm">
        <Sparkles className="size-4 text-muted-foreground" />
        <Text variant="muted">Structured by</Text>
        <Badge variant="secondary">{ai.model}</Badge>
      </Stack>
      {rows ? <AiRowsTable rows={rows} /> : null}
      <CodeBlock value={JSON.stringify(ai.data, null, 2)} maxHeight="max-h-[32rem]" />
    </Stack>
  )
}
