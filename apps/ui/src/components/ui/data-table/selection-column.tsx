import { Checkbox } from '@/components/ui/checkbox'
import { createDataTableHelper, type TDataTableColumns } from './features'

export const SELECTION_COLUMN_ID = 'select'

export function withSelectionColumn<TData extends object>(
  columns: TDataTableColumns<TData>,
): TDataTableColumns<TData> {
  const helper = createDataTableHelper<TData>()
  const select = helper.display({
    id: SELECTION_COLUMN_ID,
    header: ({ table }) => (
      <Checkbox
        aria-label="Select all rows on this page"
        checked={table.getIsAllPageRowsSelected()}
        indeterminate={table.getIsSomePageRowsSelected()}
        onCheckedChange={(checked) => table.toggleAllPageRowsSelected(checked)}
      />
    ),
    cell: ({ row }) => (
      <Checkbox
        aria-label="Select row"
        checked={row.getIsSelected()}
        onCheckedChange={(checked) => row.toggleSelected(checked)}
        onClick={(event) => event.stopPropagation()}
      />
    ),
  })
  return helper.columns([select, ...columns])
}
