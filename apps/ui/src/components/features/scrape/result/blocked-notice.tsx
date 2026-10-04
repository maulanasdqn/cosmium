import { ShieldAlert } from 'lucide-react'

import { Alert, AlertDescription, AlertTitle } from '@/components/ui/alert'
import { Badge } from '@/components/ui/badge'
import { Stack } from '@/components/ui/stack'
import { Text } from '@/components/ui/typography'

const statusHints: Record<number, string> = {
  403: 'The server rejected the request (Forbidden).',
  429: 'Too many requests. The server is rate-limiting this IP.',
  503: 'The server is temporarily unavailable or showing a challenge page.',
}

export function BlockedNotice({ httpStatus }: { httpStatus: number }) {
  const hint = statusHints[httpStatus] ?? 'The server refused to serve the page.'
  return (
    <Alert variant="destructive">
      <ShieldAlert />
      <AlertTitle>
        <Stack direction="row" align="center" gap="sm">
          Blocked
          <Badge variant="destructive">HTTP {httpStatus}</Badge>
        </Stack>
      </AlertTitle>
      <AlertDescription>
        <Stack gap="sm">
          <Text variant="default">{hint}</Text>
          <Text variant="muted">
            This usually means the site detected automated access. Try using a residential proxy,
            switching to a different profile, or waiting before retrying. Some sites block all
            automated access regardless of fingerprint.
          </Text>
        </Stack>
      </AlertDescription>
    </Alert>
  )
}
