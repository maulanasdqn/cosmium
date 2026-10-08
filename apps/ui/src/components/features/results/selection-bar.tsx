import { Trash2, X } from 'lucide-react'
import { toast } from 'sonner'

import { useDeleteResults, type TResultSummary } from '@/apis/results'
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
import { Card, CardContent } from '@/components/ui/card'
import { Spinner } from '@/components/ui/spinner'
import { Stack } from '@/components/ui/stack'
import { Text } from '@/components/ui/typography'
import { toErrorMessage } from '@/libs/http'

export function SelectionBar({
  selected,
  onClear,
}: {
  selected: TResultSummary[]
  onClear: () => void
}) {
  const remove = useDeleteResults()
  const count = selected.length
  const label = `${count} result${count === 1 ? '' : 's'}`

  async function handleDelete() {
    try {
      const { deleted } = await remove.mutateAsync(selected.map((row) => row.id))
      toast.success(`Deleted ${deleted} result${deleted === 1 ? '' : 's'}`)
      onClear()
    } catch (cause) {
      toast.error('Could not delete the results', { description: toErrorMessage(cause) })
    }
  }

  return (
    <Card size="sm" className="sticky top-16 z-10 border-primary/40 bg-card/95 backdrop-blur">
      <CardContent>
        <Stack direction="row" gap="md" align="center" justify="between">
          <Text weight="medium">{label} selected</Text>
          <Stack direction="row" gap="sm" align="center">
            <Button variant="ghost" size="sm" onClick={onClear}>
              <X />
              Clear
            </Button>
            <AlertDialog>
              <AlertDialogTrigger
                render={<Button variant="destructive" size="sm" disabled={remove.isPending} />}
              >
                {remove.isPending ? <Spinner /> : <Trash2 />}
                Delete selected
              </AlertDialogTrigger>
              <AlertDialogContent>
                <AlertDialogHeader>
                  <AlertDialogTitle>Delete {label}?</AlertDialogTitle>
                  <AlertDialogDescription>
                    The saved page data, screenshots, and AI output are removed permanently.
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
          </Stack>
        </Stack>
      </CardContent>
    </Card>
  )
}
