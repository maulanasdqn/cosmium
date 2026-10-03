import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card'
import { Stack } from '@/components/ui/stack'
import { Text } from '@/components/ui/typography'
import type { TFact } from './profile-facts'

export function FactCard({ title, facts }: { title: string; facts: TFact[] }) {
  return (
    <Card size="sm">
      <CardHeader>
        <CardTitle>{title}</CardTitle>
      </CardHeader>
      <CardContent>
        <Stack gap="sm">
          {facts.map((fact) => (
            <Stack key={fact.label} direction="row" justify="between" align="start" gap="md">
              <Text variant="muted" className="shrink-0">
                {fact.label}
              </Text>
              <Text
                variant={fact.mono ? 'mono' : 'default'}
                weight="medium"
                className="min-w-0 text-right"
              >
                {fact.value}
              </Text>
            </Stack>
          ))}
        </Stack>
      </CardContent>
    </Card>
  )
}
