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
import { Spinner } from '@/components/ui/spinner'
import { Stack } from '@/components/ui/stack'
import { useSaveProfile, type TProfileWithDiagnostics } from '@/apis/profiles'
import { toErrorMessage } from '@/libs/http'
import { DiagnosticsSummary } from './diagnostics-summary'
import { DiagnosticsTable } from './diagnostics-table'
import { ProfileJson } from './profile-json'

export function RepairDialog({
  name,
  result,
  onClose,
}: {
  name: string
  result: TProfileWithDiagnostics | null
  onClose: () => void
}) {
  const save = useSaveProfile()

  function persist() {
    if (!result) {
      return
    }
    save.mutate(
      { name, profile: result.profile },
      {
        onSuccess: () => {
          toast.success(`Saved repaired ${name}`)
          onClose()
        },
        onError: (error) => toast.error(toErrorMessage(error)),
      },
    )
  }

  return (
    <Dialog open={result !== null} onOpenChange={(open) => (open ? null : onClose())}>
      <DialogContent className="sm:max-w-3xl">
        <DialogHeader>
          <DialogTitle>Repaired {name}</DialogTitle>
          <DialogDescription>
            Review the AI repair before saving. Nothing is written until you save.
          </DialogDescription>
        </DialogHeader>
        {result ? (
          <Stack gap="md">
            <DiagnosticsSummary diagnostics={result.diagnostics} />
            <DiagnosticsTable diagnostics={result.diagnostics} />
            <ProfileJson profile={result.profile} maxHeight="max-h-72" />
          </Stack>
        ) : null}
        <DialogFooter>
          <Button variant="outline" onClick={onClose}>
            Discard
          </Button>
          <Button disabled={save.isPending} onClick={persist}>
            {save.isPending ? <Spinner /> : null}
            Save repaired
          </Button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  )
}
