import { Card, CardContent } from '@/components/ui/card'
import { Spinner } from '@/components/ui/spinner'
import { Stack } from '@/components/ui/stack'
import { Text } from '@/components/ui/typography'
import { useElapsed } from '../use-elapsed'

export function SimplePending() {
  const seconds = Math.floor(useElapsed(true) / 1000)
  return (
    <Card>
      <CardContent>
        <Stack direction="row" align="center" gap="md">
          <Spinner className="size-5" />
          <Stack gap="none">
            <Text weight="medium">Opening the page in a stealth browser… {seconds}s</Text>
            <Text variant="muted">Most pages take 2 to 10 seconds.</Text>
          </Stack>
        </Stack>
      </CardContent>
    </Card>
  )
}
