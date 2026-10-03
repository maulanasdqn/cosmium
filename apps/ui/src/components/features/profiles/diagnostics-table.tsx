import { DataTable, SortableHeader, createDataTableHelper } from '@/components/ui/data-table'
import { InlineCode, Text } from '@/components/ui/typography'
import type { TDiagnostic } from '@/apis/profiles'
import { SeverityBadge } from './severity-badge'

const helper = createDataTableHelper<TDiagnostic>()

const columns = helper.columns([
  helper.accessor('severity', {
    header: ({ column }) => <SortableHeader column={column} title="Severity" />,
    cell: ({ row }) => <SeverityBadge severity={row.original.severity} />,
  }),
  helper.accessor('field', {
    header: ({ column }) => <SortableHeader column={column} title="Field" />,
    cell: ({ row }) => <InlineCode>{row.original.field}</InlineCode>,
  }),
  helper.accessor('message', {
    header: 'Message',
    cell: ({ row }) => <Text className="whitespace-normal">{row.original.message}</Text>,
  }),
])

export function DiagnosticsTable({ diagnostics }: { diagnostics: TDiagnostic[] }) {
  return (
    <DataTable
      columns={columns}
      data={diagnostics}
      searchPlaceholder="Filter diagnostics…"
      emptyTitle="No problems"
      emptyDescription="This profile is internally consistent."
    />
  )
}
