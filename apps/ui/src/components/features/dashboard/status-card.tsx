import type { ReactNode } from 'react'

import { Card, CardAction, CardContent, CardDescription, CardHeader } from '@/components/ui/card'
import { Skeleton } from '@/components/ui/skeleton'
import { Stack } from '@/components/ui/stack'
import { Heading, Text } from '@/components/ui/typography'

export type TStatusCardProps = {
  title: string
  icon: ReactNode
  value: ReactNode
  badge?: ReactNode
  hint?: ReactNode
  loading?: boolean
}

export function StatusCard({ title, icon, value, badge, hint, loading = false }: TStatusCardProps) {
  return (
    <Card size="sm">
      <CardHeader>
        <CardDescription>{title}</CardDescription>
        <CardAction>{icon}</CardAction>
      </CardHeader>
      <CardContent>
        {loading ? (
          <Stack gap="sm">
            <Skeleton className="h-7 w-24" />
            <Skeleton className="h-4 w-32" />
          </Stack>
        ) : (
          <Stack gap="sm">
            <Stack direction="row" align="center" gap="sm" wrap>
              <Heading level={2}>{value}</Heading>
              {badge}
            </Stack>
            {hint ? <Text variant="small">{hint}</Text> : null}
          </Stack>
        )}
      </CardContent>
    </Card>
  )
}
