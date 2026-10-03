import { useMemo, useState } from 'react'

import type { TFingerprintResult } from '@/apis/tests'
import { DataTable } from '@/components/ui/data-table'
import { Stack } from '@/components/ui/stack'
import { ToggleGroup, ToggleGroupItem } from '@/components/ui/toggle-group'
import { FingerprintSummary } from './fingerprint-summary'
import { probeColumns } from './probe-columns'
import { filterProbeRows, isProbeFilter, toProbeRows, type TProbeFilter } from './probe-rows'

export function FingerprintResults({ result }: { result: TFingerprintResult }) {
  const [filter, setFilter] = useState<TProbeFilter>('all')
  const rows = useMemo(() => toProbeRows(result.probes), [result])
  const visible = useMemo(() => filterProbeRows(rows, filter), [rows, filter])

  return (
    <Stack gap="md">
      <FingerprintSummary result={result} />
      <DataTable
        columns={probeColumns}
        data={visible}
        getRowId={(row) => row.id}
        pageSize={15}
        searchPlaceholder="Search probes…"
        emptyTitle="No probes match this filter"
        toolbar={
          <ToggleGroup
            variant="outline"
            size="sm"
            value={[filter]}
            onValueChange={(value) => {
              const next = value.find(isProbeFilter)
              if (next) {
                setFilter(next)
              }
            }}
          >
            <ToggleGroupItem value="all">All</ToggleGroupItem>
            <ToggleGroupItem value="failed">Failed</ToggleGroupItem>
            <ToggleGroupItem value="passed">Passed</ToggleGroupItem>
          </ToggleGroup>
        }
      />
    </Stack>
  )
}
