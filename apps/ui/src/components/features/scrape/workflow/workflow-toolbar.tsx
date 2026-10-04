import { useState } from 'react'
import { ClipboardPaste, Copy, Sparkles, Trash2 } from 'lucide-react'
import { toast } from 'sonner'

import {
  AlertDialog,
  AlertDialogAction,
  AlertDialogCancel,
  AlertDialogContent,
  AlertDialogDescription,
  AlertDialogFooter,
  AlertDialogHeader,
  AlertDialogTitle,
  AlertDialogTrigger,
} from '@/components/ui/alert-dialog'
import { Button } from '@/components/ui/button'
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuTrigger,
} from '@/components/ui/dropdown-menu'
import { Stack } from '@/components/ui/stack'
import { Text } from '@/components/ui/typography'
import { ImportWorkflowDialog } from './import-workflow-dialog'
import { workflowPresets } from './presets'
import type { TWorkflowStep } from './types'
import { setSteps } from './workflow-store'

export function WorkflowToolbar({ steps }: { steps: TWorkflowStep[] }) {
  const [importOpen, setImportOpen] = useState(false)
  const hasSteps = steps.length > 0

  async function copyJson() {
    try {
      await navigator.clipboard.writeText(JSON.stringify(steps, null, 2))
      toast.success('Workflow copied to the clipboard')
    } catch {
      toast.error('Could not reach the clipboard', {
        description: 'Your browser blocked the copy. Select the JSON manually instead.',
      })
    }
  }

  return (
    <Stack direction="row" gap="sm" wrap>
      <DropdownMenu>
        <DropdownMenuTrigger render={<Button variant="outline" size="sm" />}>
          <Sparkles />
          Start from a preset
        </DropdownMenuTrigger>
        <DropdownMenuContent align="start" className="w-80">
          {workflowPresets.map((preset) => (
            <DropdownMenuItem
              key={preset.id}
              onClick={() => {
                setSteps(structuredClone(preset.steps))
                toast.success(`Loaded "${preset.label}"`)
              }}
            >
              <Stack gap="none">
                <Text weight="medium" inline>
                  {preset.label}
                </Text>
                <Text variant="small" inline>
                  {preset.description}
                </Text>
              </Stack>
            </DropdownMenuItem>
          ))}
        </DropdownMenuContent>
      </DropdownMenu>

      <Button variant="outline" size="sm" disabled={!hasSteps} onClick={() => void copyJson()}>
        <Copy />
        Copy JSON
      </Button>

      <Button variant="outline" size="sm" onClick={() => setImportOpen(true)}>
        <ClipboardPaste />
        Paste JSON
      </Button>
      <ImportWorkflowDialog open={importOpen} onOpenChange={setImportOpen} />

      {hasSteps ? (
        <AlertDialog>
          <AlertDialogTrigger render={<Button variant="ghost" size="sm" />}>
            <Trash2 />
            Clear
          </AlertDialogTrigger>
          <AlertDialogContent>
            <AlertDialogHeader>
              <AlertDialogTitle>Clear every step?</AlertDialogTitle>
              <AlertDialogDescription>
                This removes all {steps.length} steps from the builder.
              </AlertDialogDescription>
            </AlertDialogHeader>
            <AlertDialogFooter>
              <AlertDialogCancel>Cancel</AlertDialogCancel>
              <AlertDialogAction variant="destructive" onClick={() => setSteps([])}>
                Clear
              </AlertDialogAction>
            </AlertDialogFooter>
          </AlertDialogContent>
        </AlertDialog>
      ) : null}
    </Stack>
  )
}
