import { useMemo } from 'react'

import { createDataTableHelper, DataTable, SortableHeader } from '@/components/ui/data-table'
import { Text } from '@/components/ui/typography'
import { columnKeys, formatCell, titleCase, type TAiRow } from './ai-format'

const helper = createDataTableHelper<TAiRow>()

export function AiRowsTable({ rows }: { rows: TAiRow[] }) {
  const keys = useMemo(() => columnKeys(rows), [rows])
  const columns = useMemo(
    () =>
      helper.columns(
        keys.map((key) =>
          helper.accessor((row) => formatCell(row[key]), {
            id: key,
            header: ({ column }) => <SortableHeader column={column} title={titleCase(key)} />,
            cell: ({ getValue }) => (
              <Text className="max-w-xs break-words">{String(getValue())}</Text>
            ),
          }),
        ),
      ),
    [keys],
  )
  return (
    <DataTable
      columns={columns}
      data={rows}
      searchPlaceholder="Filter rows…"
      emptyTitle="No rows"
      pageSize={15}
    />
  )
}
