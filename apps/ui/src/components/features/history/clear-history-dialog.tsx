import { Trash2 } from 'lucide-react'
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
import { clearRuns } from '@/stores/history'

export function ClearHistoryDialog({ count }: { count: number }) {
  return (
    <AlertDialog>
      <AlertDialogTrigger render={<Button variant="outline" disabled={count === 0} />}>
        <Trash2 />
        Clear history
      </AlertDialogTrigger>
      <AlertDialogContent>
        <AlertDialogHeader>
          <AlertDialogTitle>Clear run history?</AlertDialogTitle>
          <AlertDialogDescription>
            This removes all {count} saved runs from this browser. It cannot be undone.
          </AlertDialogDescription>
        </AlertDialogHeader>
        <AlertDialogFooter>
          <AlertDialogCancel>Cancel</AlertDialogCancel>
          <AlertDialogAction
            variant="destructive"
            onClick={() => {
              clearRuns()
              toast.success('History cleared')
            }}
          >
            Clear history
          </AlertDialogAction>
        </AlertDialogFooter>
      </AlertDialogContent>
    </AlertDialog>
  )
}
