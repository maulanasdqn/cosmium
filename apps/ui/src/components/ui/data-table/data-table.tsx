import * as React from 'react'
import { SearchIcon } from 'lucide-react'
import { useTable } from '@tanstack/react-table'

import { cn } from '@/libs/utils'
import { Empty, EmptyDescription, EmptyHeader, EmptyTitle } from '@/components/ui/empty'
import { InputGroup, InputGroupAddon, InputGroupInput } from '@/components/ui/input-group'
import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from '@/components/ui/table'
import { DataTablePagination } from './data-table-pagination'
import { dataTableFeatures, type TDataTableColumns } from './features'
import { SELECTION_COLUMN_ID, withSelectionColumn } from './selection-column'

export type TDataTableProps<TData extends object> = {
  columns: TDataTableColumns<TData>
  data: TData[]
  getRowId?: (row: TData) => string
  searchPlaceholder?: string
  emptyTitle?: string
  emptyDescription?: string
  toolbar?: React.ReactNode
  pageSize?: number
  onRowClick?: (row: TData) => void
  enableSelection?: boolean
  selectionBar?: (selected: TData[], clear: () => void) => React.ReactNode
}

export function DataTable<TData extends object>({
  columns,
  data,
  getRowId,
  searchPlaceholder = 'Search…',
  emptyTitle = 'Nothing here yet',
  emptyDescription,
  toolbar,
  pageSize = 10,
  onRowClick,
  enableSelection = false,
  selectionBar,
}: TDataTableProps<TData>) {
  const initialState = React.useMemo(() => ({ pagination: { pageIndex: 0, pageSize } }), [pageSize])
  const tableColumns = React.useMemo(
    () => (enableSelection ? withSelectionColumn(columns) : columns),
    [columns, enableSelection],
  )
  const table = useTable(
    {
      features: dataTableFeatures,
      columns: tableColumns,
      data,
      getRowId,
      initialState,
      globalFilterFn: 'includesString',
      getColumnCanGlobalFilter: (column) => column.id !== SELECTION_COLUMN_ID,
    },
    (state) => ({
      globalFilter: state.globalFilter,
      pagination: state.pagination,
      rowSelection: state.rowSelection,
    }),
  )
  const rows = table.getRowModel().rows
  const columnCount = table.getAllLeafColumns().length
  const selected = table.getSelectedRowModel().rows.map((row) => row.original)

  return (
    <div data-slot="data-table" className="flex flex-col gap-3">
      <div className="flex flex-col gap-2 sm:flex-row sm:items-center sm:justify-between">
        <InputGroup className="sm:max-w-xs">
          <InputGroupAddon>
            <SearchIcon />
          </InputGroupAddon>
          <InputGroupInput
            placeholder={searchPlaceholder}
            value={String(table.state.globalFilter ?? '')}
            onChange={(event) => table.setGlobalFilter(event.target.value)}
          />
        </InputGroup>
        {toolbar ? <div className="flex flex-wrap items-center gap-2">{toolbar}</div> : null}
      </div>
      {selectionBar && selected.length > 0
        ? selectionBar(selected, () => table.resetRowSelection())
        : null}
      <div className="overflow-hidden rounded-lg border">
        <Table>
          <TableHeader>
            {table.getHeaderGroups().map((group) => (
              <TableRow key={group.id}>
                {group.headers.map((header) => (
                  <TableHead key={header.id}>
                    {header.isPlaceholder ? null : <table.FlexRender header={header} />}
                  </TableHead>
                ))}
              </TableRow>
            ))}
          </TableHeader>
          <TableBody>
            {rows.length === 0 ? (
              <TableRow>
                <TableCell colSpan={columnCount}>
                  <Empty className="border-0 py-10">
                    <EmptyHeader>
                      <EmptyTitle>{emptyTitle}</EmptyTitle>
                      {emptyDescription ? (
                        <EmptyDescription>{emptyDescription}</EmptyDescription>
                      ) : null}
                    </EmptyHeader>
                  </Empty>
                </TableCell>
              </TableRow>
            ) : (
              rows.map((row) => (
                <TableRow
                  key={row.id}
                  className={cn(onRowClick && 'cursor-pointer')}
                  onClick={onRowClick ? () => onRowClick(row.original) : undefined}
                >
                  {row.getAllCells().map((cell) => (
                    <TableCell key={cell.id}>
                      <table.FlexRender cell={cell} />
                    </TableCell>
                  ))}
                </TableRow>
              ))
            )}
          </TableBody>
        </Table>
      </div>
      <DataTablePagination
        pageIndex={table.state.pagination.pageIndex}
        pageCount={table.getPageCount()}
        total={table.getFilteredRowModel().rows.length}
        canPrevious={table.getCanPreviousPage()}
        canNext={table.getCanNextPage()}
        onPrevious={() => table.previousPage()}
        onNext={() => table.nextPage()}
      />
    </div>
  )
}
