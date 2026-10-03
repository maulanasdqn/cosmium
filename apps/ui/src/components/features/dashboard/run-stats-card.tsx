import { Progress, ProgressLabel, ProgressValue } from '@/components/ui/progress'
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card'
import { Stack } from '@/components/ui/stack'
import { useRuns, type TRunStatus } from '@/stores/history'
import { percentOf } from '@/components/features/runs'

const ROWS: { status: TRunStatus; label: string }[] = [
  { status: 'success', label: 'Success' },
  { status: 'blocked', label: 'Blocked' },
  { status: 'failed', label: 'Failed' },
]

export function RunStatsCard() {
  const runs = useRuns()
  const total = runs.length
  const count = (status: TRunStatus) => runs.filter((run) => run.status === status).length

  return (
    <Card>
      <CardHeader>
        <CardTitle>Run outcomes</CardTitle>
        <CardDescription>
          {total === 0 ? 'No scrapes recorded yet' : `${total} scrapes recorded in this browser`}
        </CardDescription>
      </CardHeader>
      <CardContent>
        <Stack gap="lg">
          {ROWS.map((row) => (
            <Progress key={row.status} value={percentOf(count(row.status), total)}>
              <ProgressLabel>
                {row.label} · {count(row.status)}
              </ProgressLabel>
              <ProgressValue />
            </Progress>
          ))}
        </Stack>
      </CardContent>
    </Card>
  )
}
