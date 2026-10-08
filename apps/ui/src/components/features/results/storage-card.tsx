import { Brush, Database } from 'lucide-react'
import { toast } from 'sonner'

import { usePruneResults, useResultStats } from '@/apis/results'
import { formatBytes } from '@/components/features/runs'
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
import { Skeleton } from '@/components/ui/skeleton'
import { Spinner } from '@/components/ui/spinner'
import { Stack } from '@/components/ui/stack'
import { Text } from '@/components/ui/typography'
import { toErrorMessage } from '@/libs/http'

function policyText(maxResults: number, maxAgeDays: number): string {
  const limits: string[] = []
  if (maxResults > 0) {
    limits.push(`newest ${maxResults}`)
  }
  if (maxAgeDays > 0) {
    limits.push(`last ${maxAgeDays} days`)
  }
  return limits.length === 0 ? 'Kept forever' : `Keeps the ${limits.join(', within the ')}`
}

export function StorageCard() {
  const stats = useResultStats()
  const prune = usePruneResults()

  if (stats.isPending) {
    return <Skeleton className="h-20 w-full" />
  }
  if (!stats.data) {
    return null
  }

  async function handlePrune() {
    try {
      const { deleted } = await prune.mutateAsync()
      toast.success(
        deleted === 0 ? 'Nothing to prune' : `Removed ${deleted} result${deleted === 1 ? '' : 's'}`,
      )
    } catch (cause) {
      toast.error('Could not prune results', { description: toErrorMessage(cause) })
    }
  }

  const { count, bytes, max_results, max_age_days } = stats.data

  return (
    <Card size="sm">
      <CardContent>
        <Stack direction="responsive" gap="md" justify="between" align="center">
          <Stack direction="row" gap="md" align="center">
            <Database className="size-4 shrink-0 text-muted-foreground" />
            <Stack gap="none">
              <Text weight="medium">
                {count} saved · {formatBytes(bytes)} on disk
              </Text>
              <Text variant="muted">{policyText(max_results, max_age_days)}</Text>
            </Stack>
          </Stack>
          <AlertDialog>
            <AlertDialogTrigger
              render={<Button variant="outline" size="sm" disabled={prune.isPending} />}
            >
              {prune.isPending ? <Spinner /> : <Brush />}
              Prune now
            </AlertDialogTrigger>
            <AlertDialogContent>
              <AlertDialogHeader>
                <AlertDialogTitle>Apply the retention policy now?</AlertDialogTitle>
                <AlertDialogDescription>
                  Results outside {policyText(max_results, max_age_days).toLowerCase()} are deleted
                  permanently.
                </AlertDialogDescription>
              </AlertDialogHeader>
              <AlertDialogFooter>
                <AlertDialogCancel>Cancel</AlertDialogCancel>
                <AlertDialogAction onClick={() => void handlePrune()}>Prune</AlertDialogAction>
              </AlertDialogFooter>
            </AlertDialogContent>
          </AlertDialog>
        </Stack>
      </CardContent>
    </Card>
  )
}
