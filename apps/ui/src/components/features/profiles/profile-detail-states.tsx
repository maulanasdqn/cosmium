import { Link } from '@tanstack/react-router'
import { ArrowLeft, SearchX } from 'lucide-react'

import { Alert, AlertDescription, AlertTitle } from '@/components/ui/alert'
import { Button } from '@/components/ui/button'
import {
  Empty,
  EmptyContent,
  EmptyDescription,
  EmptyHeader,
  EmptyMedia,
  EmptyTitle,
} from '@/components/ui/empty'
import { Skeleton } from '@/components/ui/skeleton'
import { Grid, Stack } from '@/components/ui/stack'
import { toErrorMessage } from '@/libs/http'
import { isNotFound, isUnreadable } from './api-errors'

function BackButton() {
  return (
    <Button variant="outline" render={<Link to="/profiles" />}>
      <ArrowLeft />
      Back to profiles
    </Button>
  )
}

export function ProfileDetailLoading() {
  return (
    <Stack gap="md">
      <Skeleton className="h-9 w-80" />
      <Grid columns={2}>
        <Skeleton className="h-44" />
        <Skeleton className="h-44" />
        <Skeleton className="h-44" />
        <Skeleton className="h-44" />
      </Grid>
    </Stack>
  )
}

export function ProfileDetailError({ name, error }: { name: string; error: unknown }) {
  if (isNotFound(error)) {
    return (
      <Empty className="border">
        <EmptyHeader>
          <EmptyMedia variant="icon">
            <SearchX />
          </EmptyMedia>
          <EmptyTitle>Profile not found</EmptyTitle>
          <EmptyDescription>There is no profile named {name}.</EmptyDescription>
        </EmptyHeader>
        <EmptyContent>
          <BackButton />
        </EmptyContent>
      </Empty>
    )
  }
  return (
    <Stack gap="md" align="start">
      <Alert variant="destructive">
        <AlertTitle>
          {isUnreadable(error) ? 'This profile cannot be read' : 'Could not load the profile'}
        </AlertTitle>
        <AlertDescription>{toErrorMessage(error)}</AlertDescription>
      </Alert>
      <BackButton />
    </Stack>
  )
}
