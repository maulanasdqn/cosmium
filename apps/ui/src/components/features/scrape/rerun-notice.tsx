import { RotateCcw } from 'lucide-react'

import { Alert, AlertDescription, AlertTitle } from '@/components/ui/alert'
import { formatRelative } from '@/components/features/runs'

export function RerunNotice({ url, startedAt }: { url: string; startedAt: string }) {
  return (
    <Alert>
      <RotateCcw />
      <AlertTitle>Re-running {url}</AlertTitle>
      <AlertDescription>
        The form is filled from the run started {formatRelative(startedAt)}. Adjust anything before
        running it again.
      </AlertDescription>
    </Alert>
  )
}
