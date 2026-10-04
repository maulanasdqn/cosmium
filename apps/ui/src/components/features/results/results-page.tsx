import { Link, useNavigate } from '@tanstack/react-router'
import { Plus } from 'lucide-react'

import { useResults, type TResultSummary } from '@/apis/results'
import { Alert, AlertDescription, AlertTitle } from '@/components/ui/alert'
import { Button } from '@/components/ui/button'
import { DataTable } from '@/components/ui/data-table'
import { Page, PageHeader } from '@/components/ui/page'
import { Skeleton } from '@/components/ui/skeleton'
import { toErrorMessage } from '@/libs/http'
import { resultColumns } from './result-columns'

const EMPTY: TResultSummary[] = []

export function ResultsPage() {
  const results = useResults()
  const navigate = useNavigate()
  return (
    <Page>
      <PageHeader
        title="Results"
        description="Every scrape is saved on the engine with its screenshot, data, and AI output."
        actions={
          <Button render={<Link to="/scrape" />}>
            <Plus />
            New scrape
          </Button>
        }
      />
      {results.isPending ? <Skeleton className="h-64 w-full" /> : null}
      {results.isError ? (
        <Alert variant="destructive">
          <AlertTitle>Could not load results</AlertTitle>
          <AlertDescription>{toErrorMessage(results.error)}</AlertDescription>
        </Alert>
      ) : null}
      {results.data ? (
        <DataTable
          columns={resultColumns}
          data={results.data ?? EMPTY}
          getRowId={(row) => row.id}
          searchPlaceholder="Search pages and profiles…"
          emptyTitle="No saved results yet"
          emptyDescription="Run a scrape and it will be saved here."
          pageSize={15}
          onRowClick={(row) => void navigate({ to: '/scrape/$id', params: { id: row.id } })}
        />
      ) : null}
    </Page>
  )
}
