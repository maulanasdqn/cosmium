import { useNavigate } from '@tanstack/react-router'
import { Trash2 } from 'lucide-react'
import { toast } from 'sonner'

import { useDeleteResult } from '@/apis/results'
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
import { toErrorMessage } from '@/libs/http'

export function DeleteResultButton({ id }: { id: string }) {
  const remove = useDeleteResult()
  const navigate = useNavigate()

  async function handleDelete() {
    try {
      await remove.mutateAsync(id)
      toast.success('Result deleted')
      await navigate({ to: '/results' })
    } catch (cause) {
      toast.error('Could not delete the result', { description: toErrorMessage(cause) })
    }
  }

  return (
    <AlertDialog>
      <AlertDialogTrigger render={<Button variant="destructive" size="sm" />}>
        <Trash2 />
        Delete
      </AlertDialogTrigger>
      <AlertDialogContent>
        <AlertDialogHeader>
          <AlertDialogTitle>Delete this result?</AlertDialogTitle>
          <AlertDialogDescription>
            The saved page data, screenshot, and AI output are removed permanently.
          </AlertDialogDescription>
        </AlertDialogHeader>
        <AlertDialogFooter>
          <AlertDialogCancel>Cancel</AlertDialogCancel>
          <AlertDialogAction variant="destructive" onClick={() => void handleDelete()}>
            Delete
          </AlertDialogAction>
        </AlertDialogFooter>
      </AlertDialogContent>
    </AlertDialog>
  )
}
