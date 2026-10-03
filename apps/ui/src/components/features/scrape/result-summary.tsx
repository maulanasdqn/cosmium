import { ExternalLink } from 'lucide-react'

import type { TScrapeResult } from '@/apis/scrape'
import { Badge } from '@/components/ui/badge'
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card'
import { Grid, Stack } from '@/components/ui/stack'
import { Tooltip, TooltipContent, TooltipTrigger } from '@/components/ui/tooltip'
import { Text } from '@/components/ui/typography'
import { formatDuration } from '@/components/features/runs'
import { RunStatusBadge } from '@/components/features/runs'

function Stat({ label, value }: { label: string; value: string }) {
  return (
    <Stack gap="xs">
      <Text variant="small">{label}</Text>
      <Text weight="semibold" truncate>
        {value}
      </Text>
    </Stack>
  )
}

export function ResultSummary({ result }: { result: TScrapeResult }) {
  return (
    <Card>
      <CardHeader>
        <Stack direction="row" align="center" gap="sm" wrap>
          <CardTitle>Result</CardTitle>
          <RunStatusBadge status={result.blocked ? 'blocked' : 'success'} />
          <Badge variant="outline">HTTP {result.http_status}</Badge>
        </Stack>
        <CardDescription>
          <Tooltip>
            <TooltipTrigger render={<Text inline variant="muted" truncate />}>
              <ExternalLink className="mr-1 inline size-3" />
              {result.final_url}
            </TooltipTrigger>
            <TooltipContent>{result.final_url}</TooltipContent>
          </Tooltip>
        </CardDescription>
      </CardHeader>
      <CardContent>
        <Grid columns={4}>
          <Stat label="Elapsed" value={formatDuration(result.elapsed_ms)} />
          <Stat label="Attempts" value={String(result.attempts)} />
          <Stat label="Cookies" value={String(result.cookies_count)} />
          <Stat label="Proxy" value={result.proxy_used ?? 'Direct'} />
        </Grid>
      </CardContent>
    </Card>
  )
}
