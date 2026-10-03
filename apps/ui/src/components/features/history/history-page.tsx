import { useMemo, useState } from 'react'

import { DataTable } from '@/components/ui/data-table'
import { Page, PageHeader } from '@/components/ui/page'
import { useRuns, type TRunEntry } from '@/stores/history'
import { ClearHistoryDialog } from './clear-history-dialog'
import { historyColumns } from './history-columns'
import { HistoryEmpty } from './history-empty'
import { StatusFilter, type TStatusFilter } from './status-filter'

const getRunId = (run: TRunEntry) => run.id

export function HistoryPage() {
  const runs = useRuns()
  const [status, setStatus] = useState<TStatusFilter>('all')
  const visibleRuns = useMemo(
    () => (status === 'all' ? runs : runs.filter((run) => run.status === status)),
    [runs, status],
  )

  return (
    <Page>
      <PageHeader
        title="Run history"
        description="Scrapes run from this browser, newest first. Stored locally, up to 200 runs."
        actions={<ClearHistoryDialog count={runs.length} />}
      />
      {runs.length === 0 ? (
        <HistoryEmpty />
      ) : (
        <DataTable
          columns={historyColumns}
          data={visibleRuns}
          getRowId={getRunId}
          searchPlaceholder="Search URL, profile, or proxy…"
          emptyTitle="No runs match"
          emptyDescription="Try another status or search."
          toolbar={<StatusFilter value={status} onChange={setStatus} />}
        />
      )}
    </Page>
  )
}
