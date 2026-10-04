import { CircleAlert } from 'lucide-react'

import { Alert, AlertDescription, AlertTitle } from '@/components/ui/alert'
import { JsonBlock } from '@/components/ui/json-block'
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogHeader,
  DialogTitle,
} from '@/components/ui/dialog'
import { Stack } from '@/components/ui/stack'
import { Text } from '@/components/ui/typography'
import type { TRunEntry } from '@/stores/history'
import { formatDuration, formatExact } from '@/components/features/runs'
import { RunStatusBadge } from '@/components/features/runs'

export function RunDetailsDialog({
  run,
  open,
  onOpenChange,
}: {
  run: TRunEntry
  open: boolean
  onOpenChange: (open: boolean) => void
}) {
  return (
    <Dialog open={open} onOpenChange={onOpenChange}>
      <DialogContent className="sm:max-w-2xl">
        <DialogHeader>
          <DialogTitle>Run details</DialogTitle>
          <DialogDescription>{formatExact(run.startedAt)}</DialogDescription>
        </DialogHeader>
        <Stack gap="md">
          <Stack direction="row" align="center" gap="sm" wrap>
            <RunStatusBadge status={run.status} />
            <Text variant="muted">
              {run.httpStatus !== null ? `HTTP ${run.httpStatus} · ` : ''}
              {formatDuration(run.elapsedMs)}
            </Text>
          </Stack>
          {run.finalUrl ? <Text variant="mono">{run.finalUrl}</Text> : null}
          {run.error ? (
            <Alert variant="destructive">
              <CircleAlert />
              <AlertTitle>Error</AlertTitle>
              <AlertDescription>{run.error}</AlertDescription>
            </Alert>
          ) : null}
          <Text weight="medium">Request</Text>
          <JsonBlock data={run.payload} maxHeight="max-h-80" />
        </Stack>
      </DialogContent>
    </Dialog>
  )
}
