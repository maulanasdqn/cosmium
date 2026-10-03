import { Badge } from '@/components/ui/badge'
import { SortableHeader, createDataTableHelper } from '@/components/ui/data-table'
import { Text } from '@/components/ui/typography'
import type { TProbeRow } from './probe-rows'
import { ProbeStatusBadge } from './status-badges'
import { ValueCell } from './value-cell'

const helper = createDataTableHelper<TProbeRow>()

export const probeColumns = helper.columns([
  helper.accessor('id', {
    header: ({ column }) => <SortableHeader column={column} title="Probe" />,
    cell: ({ getValue }) => (
      <Text variant="mono" inline>
        {getValue()}
      </Text>
    ),
  }),
  helper.accessor('category', {
    header: ({ column }) => <SortableHeader column={column} title="Category" />,
    cell: ({ getValue }) => <Badge variant="outline">{getValue()}</Badge>,
  }),
  helper.accessor('status', {
    header: ({ column }) => <SortableHeader column={column} title="Result" />,
    cell: ({ getValue }) => <ProbeStatusBadge status={getValue()} />,
  }),
  helper.accessor('got', {
    header: 'Got',
    cell: ({ getValue }) => <ValueCell value={getValue()} />,
  }),
  helper.accessor('expected', {
    header: 'Expected',
    cell: ({ getValue }) => <ValueCell value={getValue()} />,
  }),
  helper.accessor('error', {
    header: 'Error',
    cell: ({ getValue }) => <ValueCell value={getValue() ?? ''} />,
  }),
])
