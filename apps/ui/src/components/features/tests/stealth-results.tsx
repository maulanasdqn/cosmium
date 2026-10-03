import type { TStealthCheck, TStealthResult } from '@/apis/tests'
import { Badge } from '@/components/ui/badge'
import {
  Card,
  CardAction,
  CardContent,
  CardFooter,
  CardHeader,
  CardTitle,
} from '@/components/ui/card'
import { Grid, Stack } from '@/components/ui/stack'
import { Text } from '@/components/ui/typography'
import { VerdictBadge } from './status-badges'

const TARGET_LABELS: Record<string, string> = {
  creepjs: 'CreepJS',
  pixelscan: 'Pixelscan',
  browserleaks: 'BrowserLeaks',
  botcheck: 'Bot-check URL',
}

function formatDuration(ms: number): string {
  return ms < 1000 ? `${ms} ms` : `${(ms / 1000).toFixed(1)} s`
}

function CheckCard({ check }: { check: TStealthCheck }) {
  return (
    <Card size="sm">
      <CardHeader>
        <CardTitle>{TARGET_LABELS[check.target] ?? check.target}</CardTitle>
        <CardAction>
          <VerdictBadge verdict={check.verdict} />
        </CardAction>
      </CardHeader>
      <CardContent>
        <Text variant="muted">{check.detail}</Text>
      </CardContent>
      <CardFooter>
        <Text variant="small">Finished in {formatDuration(check.duration_ms)}</Text>
      </CardFooter>
    </Card>
  )
}

export function StealthResults({ result }: { result: TStealthResult }) {
  const count = (verdict: TStealthCheck['verdict']) =>
    result.results.filter((check) => check.verdict === verdict).length

  return (
    <Stack gap="md">
      <Stack direction="row" gap="sm" wrap>
        <Badge variant="outline">{count('pass')} passed</Badge>
        <Badge variant="outline">{count('warn')} warnings</Badge>
        <Badge variant="outline">{count('fail')} failed</Badge>
      </Stack>
      <Grid columns={3}>
        {result.results.map((check) => (
          <CheckCard key={check.target} check={check} />
        ))}
      </Grid>
    </Stack>
  )
}
