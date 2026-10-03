import { CircleAlert, ScanSearch } from 'lucide-react'

import { Alert, AlertDescription, AlertTitle } from '@/components/ui/alert'
import { Card, CardContent } from '@/components/ui/card'
import { Empty, EmptyDescription, EmptyHeader, EmptyMedia, EmptyTitle } from '@/components/ui/empty'
import { Spinner } from '@/components/ui/spinner'
import { Stack } from '@/components/ui/stack'
import { Text } from '@/components/ui/typography'
import { formatDuration } from '@/components/features/runs'
import { useElapsed } from './use-elapsed'

export function ResultIdle() {
  return (
    <Card>
      <CardContent>
        <Empty>
          <EmptyHeader>
            <EmptyMedia variant="icon">
              <ScanSearch />
            </EmptyMedia>
            <EmptyTitle>No result yet</EmptyTitle>
            <EmptyDescription>
              Fill in the target and run a scrape. The result shows up here.
            </EmptyDescription>
          </EmptyHeader>
        </Empty>
      </CardContent>
    </Card>
  )
}

export function ResultPending() {
  const elapsed = useElapsed(true)
  return (
    <Card aria-live="polite">
      <CardContent>
        <Stack direction="row" align="center" gap="md">
          <Spinner className="size-5" />
          <Stack gap="none" grow>
            <Text weight="medium">Scraping…</Text>
            <Text variant="muted">Launching the browser, loading, and extracting.</Text>
          </Stack>
          <Text variant="mono">{formatDuration(Math.round(elapsed))}</Text>
        </Stack>
      </CardContent>
    </Card>
  )
}

export function ResultError({ message }: { message: string }) {
  return (
    <Alert variant="destructive">
      <CircleAlert />
      <AlertTitle>Scrape failed</AlertTitle>
      <AlertDescription>{message}</AlertDescription>
    </Alert>
  )
}
