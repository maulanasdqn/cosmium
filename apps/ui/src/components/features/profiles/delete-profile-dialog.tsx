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
} from '@/components/ui/alert-dialog'
import { Spinner } from '@/components/ui/spinner'
import { useDeleteProfile } from '@/apis/profiles'
import { toErrorMessage } from '@/libs/http'

export function DeleteProfileDialog({
  name,
  open,
  onOpenChange,
  onDeleted,
}: {
  name: string
  open: boolean
  onOpenChange: (open: boolean) => void
  onDeleted?: () => void
}) {
  const remove = useDeleteProfile()

  function confirm() {
    remove.mutate(name, {
      onSuccess: () => {
        toast.success(`Deleted ${name}`)
        onOpenChange(false)
        onDeleted?.()
      },
      onError: (error) => toast.error(toErrorMessage(error)),
    })
  }

  return (
    <AlertDialog open={open} onOpenChange={onOpenChange}>
      <AlertDialogContent onClick={(event) => event.stopPropagation()}>
        <AlertDialogHeader>
          <AlertDialogTitle>Delete {name}?</AlertDialogTitle>
          <AlertDialogDescription>
            The profile file is removed from the profiles directory. This cannot be undone.
          </AlertDialogDescription>
        </AlertDialogHeader>
        <AlertDialogFooter>
          <AlertDialogCancel disabled={remove.isPending}>Cancel</AlertDialogCancel>
          <AlertDialogAction variant="destructive" disabled={remove.isPending} onClick={confirm}>
            {remove.isPending ? <Spinner /> : null}
            Delete
          </AlertDialogAction>
        </AlertDialogFooter>
      </AlertDialogContent>
    </AlertDialog>
  )
}
