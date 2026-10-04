import { useMemo } from 'react'

import { createDataTableHelper, DataTable, SortableHeader } from '@/components/ui/data-table'
import { ExternalLink } from '@/components/ui/external-link'
import { Text } from '@/components/ui/typography'
import type { TPageLink } from './page-data'

const helper = createDataTableHelper<TPageLink>()

const columns = helper.columns([
  helper.accessor('text', {
    header: ({ column }) => <SortableHeader column={column} title="Label" />,
    cell: ({ getValue }) => (
      <Text truncate className="max-w-xs">
        {getValue() || '—'}
      </Text>
    ),
  }),
  helper.accessor('href', {
    header: ({ column }) => <SortableHeader column={column} title="Address" />,
    cell: ({ getValue }) => (
      <ExternalLink href={getValue()} className="block max-w-md truncate font-mono text-xs">
        {getValue()}
      </ExternalLink>
    ),
  }),
])

export function LinksTable({ links }: { links: TPageLink[] }) {
  const data = useMemo(() => links, [links])
  return (
    <DataTable
      columns={columns}
      data={data}
      getRowId={(row) => row.href}
      searchPlaceholder="Filter links…"
      emptyTitle="No links found"
      pageSize={15}
    />
  )
}
