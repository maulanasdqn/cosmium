import { Link } from '@tanstack/react-router'

import { useResult, type TStoredResult } from '@/apis/results'
import { formatRelative } from '@/components/features/runs'
import { Button } from '@/components/ui/button'
import {
  Empty,
  EmptyContent,
  EmptyDescription,
  EmptyHeader,
  EmptyTitle,
} from '@/components/ui/empty'
import { Page, PageHeader } from '@/components/ui/page'
import { Skeleton } from '@/components/ui/skeleton'
import { Stack } from '@/components/ui/stack'
import { readPageData, readSelectorMatches } from '../simple/page-data'
import { ResultHero } from '../simple/result-hero'
import { ResultSections } from '../simple/result-sections'
import { BackButton } from '@/components/ui/back-button'
import { BlockedNotice } from './blocked-notice'
import { ResultActions } from './result-actions'
import { useSavedAi } from './use-saved-ai'

function SavedResult({ stored }: { stored: TStoredResult }) {
  const blocked = stored.result.blocked
  const { ai, available, canFormat, format } = useSavedAi(stored)
  const data = readPageData(stored.result)
  const created = new Date(stored.created_at_ms).toISOString()
  return (
    <Page>
      <Stack gap="xs">
        <BackButton fallback="/results" label="All results" />
        <PageHeader
          title="Scrape result"
          description={`Scraped ${formatRelative(created)} with ${stored.payload.profile}`}
          actions={
            <ResultActions
              stored={stored}
              showFormat={!blocked && available && ai.status === 'off'}
              canFormat={canFormat}
              onFormat={() => void format()}
            />
          }
        />
      </Stack>
      {blocked ? (
        <BlockedNotice httpStatus={stored.result.http_status} />
      ) : (
        <Stack gap="lg">
          <ResultHero
            result={stored.result}
            data={data}
            formatted={ai.status === 'done' ? ai.data : undefined}
          />
          <ResultSections
            data={data}
            matches={readSelectorMatches(stored.result)}
            extracted={stored.result.extracted}
            ai={ai}
          />
        </Stack>
      )}
    </Page>
  )
}

export function ScrapeResultPage({ id }: { id: string }) {
  const query = useResult(id)
  if (query.isPending) {
    return (
      <Page>
        <Skeleton className="h-10 w-64" />
        <Skeleton className="h-80 w-full" />
      </Page>
    )
  }
  if (!query.data) {
    return (
      <Page>
        <BackButton fallback="/results" label="All results" />
        <Empty className="border">
          <EmptyHeader>
            <EmptyTitle>Result not found</EmptyTitle>
            <EmptyDescription>It may have been deleted, or the link is wrong.</EmptyDescription>
          </EmptyHeader>
          <EmptyContent>
            <Button render={<Link to="/results" />}>Browse results</Button>
          </EmptyContent>
        </Empty>
      </Page>
    )
  }
  return <SavedResult key={query.data.id} stored={query.data} />
}
