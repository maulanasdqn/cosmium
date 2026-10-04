import { useState } from 'react'
import { toast } from 'sonner'

import { Button } from '@/components/ui/button'
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
} from '@/components/ui/dialog'
import { Textarea } from '@/components/ui/textarea'
import { Stack } from '@/components/ui/stack'
import { Text } from '@/components/ui/typography'
import { workflowSchema } from './schema'
import { setSteps } from './workflow-store'

export function ImportWorkflowDialog({
  open,
  onOpenChange,
}: {
  open: boolean
  onOpenChange: (open: boolean) => void
}) {
  const [raw, setRaw] = useState('')
  const [error, setError] = useState<string | null>(null)

  function load() {
    try {
      const parsed = workflowSchema.safeParse(JSON.parse(raw))
      if (!parsed.success) {
        const issue = parsed.error.issues[0]
        setError(
          issue ? `${issue.path.join('.') || 'workflow'}: ${issue.message}` : 'Invalid workflow',
        )
        return
      }
      setSteps(parsed.data)
      setError(null)
      setRaw('')
      onOpenChange(false)
      toast.success(`Loaded ${parsed.data.length} steps`)
    } catch {
      setError('That is not valid JSON')
    }
  }

  return (
    <Dialog open={open} onOpenChange={onOpenChange}>
      <DialogContent>
        <DialogHeader>
          <DialogTitle>Paste a workflow</DialogTitle>
          <DialogDescription>
            Paste JSON copied from another workflow. It replaces the current steps.
          </DialogDescription>
        </DialogHeader>
        <Stack gap="sm">
          <Textarea
            rows={12}
            className="font-mono text-xs"
            placeholder='[{"type":"extract","name":"titles","selector":".title","attribute":null,"limit":0}]'
            value={raw}
            onChange={(event) => setRaw(event.target.value)}
          />
          {error ? <Text variant="error">{error}</Text> : null}
        </Stack>
        <DialogFooter>
          <Button variant="outline" onClick={() => onOpenChange(false)}>
            Cancel
          </Button>
          <Button disabled={raw.trim() === ''} onClick={load}>
            Load steps
          </Button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  )
}
