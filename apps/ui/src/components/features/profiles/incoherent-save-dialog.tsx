import {
  AlertDialog,
  AlertDialogAction,
  AlertDialogCancel,
  AlertDialogContent,
  AlertDialogDescription,
  AlertDialogFooter,
  AlertDialogHeader,
  AlertDialogTitle,
} from '@/components/ui/alert-dialog'
import { Item, ItemContent, ItemDescription, ItemGroup, ItemTitle } from '@/components/ui/item'
import { Spinner } from '@/components/ui/spinner'
import { Stack } from '@/components/ui/stack'
import { InlineCode } from '@/components/ui/typography'
import type { TPendingSave } from './use-profile-save'

export function IncoherentSaveDialog({
  pending,
  busy,
  onCancel,
  onConfirm,
}: {
  pending: TPendingSave | null
  busy: boolean
  onCancel: () => void
  onConfirm: () => void
}) {
  const errors = pending?.errors ?? []
  return (
    <AlertDialog open={pending !== null} onOpenChange={(open) => (open ? null : onCancel())}>
      <AlertDialogContent>
        <AlertDialogHeader>
          <AlertDialogTitle>This profile is not coherent</AlertDialogTitle>
          <AlertDialogDescription>
            {errors.length === 1
              ? 'One value contradicts the rest of the profile.'
              : `${errors.length} values contradict the rest of the profile.`}{' '}
            Sites may notice the mismatch. Save anyway?
          </AlertDialogDescription>
        </AlertDialogHeader>
        <Stack className="max-h-64 overflow-auto">
          <ItemGroup>
            {errors.map((entry, index) => (
              <Item key={`${entry.field}-${index}`} size="sm" className="items-start">
                <ItemContent>
                  <ItemTitle>
                    <InlineCode>{entry.field}</InlineCode>
                  </ItemTitle>
                  <ItemDescription>{entry.message}</ItemDescription>
                </ItemContent>
              </Item>
            ))}
          </ItemGroup>
        </Stack>
        <AlertDialogFooter>
          <AlertDialogCancel disabled={busy}>Cancel</AlertDialogCancel>
          <AlertDialogAction variant="destructive" disabled={busy} onClick={onConfirm}>
            {busy ? <Spinner /> : null}
            Save anyway
          </AlertDialogAction>
        </AlertDialogFooter>
      </AlertDialogContent>
    </AlertDialog>
  )
}
