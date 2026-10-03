import type { TFingerprintResult } from '@/apis/tests'
import { Badge } from '@/components/ui/badge'
import {
  Card,
  CardAction,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from '@/components/ui/card'
import { Progress } from '@/components/ui/progress'
import { Stack } from '@/components/ui/stack'
import { Text } from '@/components/ui/typography'

export function FingerprintSummary({ result }: { result: TFingerprintResult }) {
  const total = result.passed + result.failed
  const rate = total === 0 ? 0 : Math.round((result.passed / total) * 100)
  const clean = result.failed === 0

  return (
    <Card>
      <CardHeader>
        <CardTitle>
          {result.passed} of {total} probes passed
        </CardTitle>
        <CardDescription>
          {clean
            ? 'Every probe matched the profile and no inconsistencies were found.'
            : `${result.failed} ${result.failed === 1 ? 'probe' : 'probes'} differ from the profile or leak an inconsistency.`}
        </CardDescription>
        <CardAction>
          {clean ? (
            <Badge className="bg-emerald-500/15 text-emerald-600 dark:text-emerald-400">
              Consistent
            </Badge>
          ) : (
            <Badge variant="destructive">Needs attention</Badge>
          )}
        </CardAction>
      </CardHeader>
      <CardContent>
        <Stack gap="xs">
          <Progress value={rate} />
          <Text variant="small">{rate}% pass rate</Text>
        </Stack>
      </CardContent>
    </Card>
  )
}
