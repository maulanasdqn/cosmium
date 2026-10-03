import { Link, useNavigate } from '@tanstack/react-router'
import { Plus, UserRound } from 'lucide-react'

import { Alert, AlertDescription, AlertTitle } from '@/components/ui/alert'
import { Button } from '@/components/ui/button'
import { DataTable } from '@/components/ui/data-table'
import {
  Empty,
  EmptyContent,
  EmptyDescription,
  EmptyHeader,
  EmptyMedia,
  EmptyTitle,
} from '@/components/ui/empty'
import { Page, PageHeader } from '@/components/ui/page'
import { Skeleton } from '@/components/ui/skeleton'
import { Stack } from '@/components/ui/stack'
import { useProfileSummaries } from '@/apis/profiles'
import { toErrorMessage } from '@/libs/http'
import { profileColumns } from './profile-columns'

function NewProfileButton() {
  return (
    <Button render={<Link to="/profiles/new" />}>
      <Plus />
      New profile
    </Button>
  )
}

function ProfilesEmpty() {
  return (
    <Empty className="border">
      <EmptyHeader>
        <EmptyMedia variant="icon">
          <UserRound />
        </EmptyMedia>
        <EmptyTitle>No profiles yet</EmptyTitle>
        <EmptyDescription>
          Generate a coherent fingerprint profile from a persona description to get started.
        </EmptyDescription>
      </EmptyHeader>
      <EmptyContent>
        <NewProfileButton />
      </EmptyContent>
    </Empty>
  )
}

function ProfilesLoading() {
  return (
    <Stack gap="sm">
      <Skeleton className="h-8 w-72" />
      <Skeleton className="h-64 w-full" />
    </Stack>
  )
}

export function ProfilesPage() {
  const summaries = useProfileSummaries()
  const navigate = useNavigate()

  return (
    <Page>
      <PageHeader
        title="Profiles"
        description="Fingerprint profiles the browser can impersonate."
        actions={<NewProfileButton />}
      />
      {summaries.isPending ? <ProfilesLoading /> : null}
      {summaries.isError ? (
        <Alert variant="destructive">
          <AlertTitle>Could not load profiles</AlertTitle>
          <AlertDescription>{toErrorMessage(summaries.error)}</AlertDescription>
        </Alert>
      ) : null}
      {summaries.data && summaries.data.length === 0 ? <ProfilesEmpty /> : null}
      {summaries.data && summaries.data.length > 0 ? (
        <DataTable
          columns={profileColumns}
          data={summaries.data}
          getRowId={(row) => row.name}
          searchPlaceholder="Search profiles…"
          onRowClick={(row) => void navigate({ to: '/profiles/$name', params: { name: row.name } })}
        />
      ) : null}
    </Page>
  )
}
