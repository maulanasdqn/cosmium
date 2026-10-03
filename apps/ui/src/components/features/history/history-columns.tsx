import { createDataTableHelper, SortableHeader } from '@/components/ui/data-table'
import { Text } from '@/components/ui/typography'
import type { TRunEntry } from '@/stores/history'
import { formatDuration } from '@/components/features/runs'
import { RunActions } from './run-actions'
import { RunStatusBadge } from '@/components/features/runs'
import { StartedCell } from './started-cell'

const helper = createDataTableHelper<TRunEntry>()

export const historyColumns = helper.columns([
  helper.accessor('startedAt', {
    header: ({ column }) => <SortableHeader column={column} title="Started" />,
    cell: ({ getValue }) => <StartedCell iso={getValue()} />,
    sortFn: 'text',
    enableGlobalFilter: false,
  }),
  helper.accessor((run) => run.payload.url, {
    id: 'url',
    header: ({ column }) => <SortableHeader column={column} title="URL" />,
    cell: ({ getValue }) => (
      <Text inline variant="mono" truncate className="block max-w-72">
        {getValue()}
      </Text>
    ),
  }),
  helper.accessor((run) => run.payload.profile, {
    id: 'profile',
    header: ({ column }) => <SortableHeader column={column} title="Profile" />,
  }),
  helper.accessor('status', {
    header: ({ column }) => <SortableHeader column={column} title="Status" />,
    cell: ({ getValue }) => <RunStatusBadge status={getValue()} />,
  }),
  helper.accessor('httpStatus', {
    header: 'HTTP',
    cell: ({ getValue }) => getValue() ?? '—',
    enableGlobalFilter: false,
  }),
  helper.accessor('elapsedMs', {
    header: ({ column }) => <SortableHeader column={column} title="Elapsed" />,
    cell: ({ getValue }) => formatDuration(getValue()),
    sortUndefined: 'last',
    enableGlobalFilter: false,
  }),
  helper.accessor('attempts', {
    header: 'Attempts',
    cell: ({ getValue }) => getValue() ?? '—',
    enableGlobalFilter: false,
  }),
  helper.accessor('proxyUsed', {
    header: 'Proxy',
    cell: ({ getValue }) => (
      <Text inline variant="muted" truncate className="block max-w-40">
        {getValue() ?? 'Direct'}
      </Text>
    ),
  }),
  helper.display({
    id: 'actions',
    header: () => null,
    cell: ({ row }) => <RunActions run={row.original} />,
  }),
])
