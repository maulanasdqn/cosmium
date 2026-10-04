import { Copy, Download, ExternalLink as ExternalLinkIcon } from 'lucide-react'
import { toast } from 'sonner'

import type { TScrapeResult } from '@/apis/scrape'
import { formatDuration, RunStatusBadge } from '@/components/features/runs'
import { Badge } from '@/components/ui/badge'
import { Button } from '@/components/ui/button'
import { Card, CardContent } from '@/components/ui/card'
import { ImagePreview } from '@/components/ui/code-block'
import { ExternalLink } from '@/components/ui/external-link'
import { Grid, Stack } from '@/components/ui/stack'
import { Heading, Text } from '@/components/ui/typography'
import { downloadJson, fileNameFor, type TPageData } from './page-data'

export function ResultHero({
  result,
  data,
  formatted,
}: {
  result: TScrapeResult
  data: TPageData
  formatted?: unknown
}) {
  const title = data.summary?.title || result.final_url
  const exportable =
    formatted === undefined
      ? { url: result.final_url, status: result.http_status, ...data }
      : formatted

  return (
    <Card>
      <CardContent>
        <Grid columns={2} gap="lg" className="items-start">
          {result.screenshot_base64 ? (
            <ImagePreview
              src={`data:image/jpeg;base64,${result.screenshot_base64}`}
              alt={`Screenshot of ${title}`}
              className="max-h-80 object-cover object-top"
            />
          ) : null}
          <Stack gap="md">
            <Stack direction="row" gap="sm" wrap>
              <RunStatusBadge status={result.blocked ? 'blocked' : 'success'} />
              <Badge variant="outline">HTTP {result.http_status}</Badge>
              <Badge variant="outline">{formatDuration(result.elapsed_ms)}</Badge>
            </Stack>
            <Heading level={3} className="line-clamp-2">
              {title}
            </Heading>
            {data.summary?.description ? (
              <Text variant="muted" className="line-clamp-3">
                {data.summary.description}
              </Text>
            ) : null}
            <Text variant="mono" className="text-muted-foreground">
              {result.final_url}
            </Text>
            {result.blocked ? (
              <Text variant="error">
                The site showed a block or challenge page. Try again, or use a proxy from More
                options in the Advanced tab.
              </Text>
            ) : null}
            <Stack direction="row" gap="sm" wrap>
              <Button
                size="sm"
                onClick={() => downloadJson(fileNameFor(result.final_url), exportable)}
              >
                <Download />
                Download JSON
              </Button>
              <Button
                size="sm"
                variant="outline"
                onClick={() => {
                  void navigator.clipboard.writeText(JSON.stringify(exportable, null, 2))
                  toast.success('Copied to clipboard')
                }}
              >
                <Copy />
                Copy JSON
              </Button>
              <Button size="sm" variant="ghost" render={<ExternalLink href={result.final_url} />}>
                <ExternalLinkIcon />
                Open page
              </Button>
            </Stack>
          </Stack>
        </Grid>
      </CardContent>
    </Card>
  )
}
