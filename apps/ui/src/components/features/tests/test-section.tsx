import * as React from 'react'
import { CircleAlert } from 'lucide-react'

import { Alert, AlertDescription, AlertTitle } from '@/components/ui/alert'
import { Card, CardContent } from '@/components/ui/card'
import { Empty, EmptyDescription, EmptyHeader, EmptyTitle } from '@/components/ui/empty'
import { Spinner } from '@/components/ui/spinner'
import { Stack } from '@/components/ui/stack'
import { Heading, Text } from '@/components/ui/typography'
import { useElapsedSeconds } from './use-elapsed-seconds'

export type TTestSectionProps = {
  title: string
  description: string
  pending: boolean
  pendingHint: string
  error: string | null
  emptyTitle: string
  emptyDescription: string
  children: React.ReactNode | null
}

function PendingState({ hint }: { hint: string }) {
  const elapsed = useElapsedSeconds(true)
  return (
    <Card>
      <CardContent>
        <Stack direction="row" gap="md" align="center">
          <Spinner className="size-5" />
          <Stack gap="xs">
            <Text weight="medium">Running… {elapsed}s elapsed</Text>
            <Text variant="muted">{hint}</Text>
          </Stack>
        </Stack>
      </CardContent>
    </Card>
  )
}

export function TestSection({
  title,
  description,
  pending,
  pendingHint,
  error,
  emptyTitle,
  emptyDescription,
  children,
}: TTestSectionProps) {
  return (
    <Stack gap="md">
      <Stack gap="xs">
        <Heading level={3}>{title}</Heading>
        <Text variant="muted">{description}</Text>
      </Stack>
      {pending ? <PendingState hint={pendingHint} /> : null}
      {!pending && error ? (
        <Alert variant="destructive">
          <CircleAlert />
          <AlertTitle>The test could not run</AlertTitle>
          <AlertDescription>{error}</AlertDescription>
        </Alert>
      ) : null}
      {!pending && children ? children : null}
      {!pending && !children && !error ? (
        <Empty className="border">
          <EmptyHeader>
            <EmptyTitle>{emptyTitle}</EmptyTitle>
            <EmptyDescription>{emptyDescription}</EmptyDescription>
          </EmptyHeader>
        </Empty>
      ) : null}
    </Stack>
  )
}
