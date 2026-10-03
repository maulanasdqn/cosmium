import { ArrowDown, ArrowUp, ArrowUpDown } from 'lucide-react'
import type { Column } from '@tanstack/react-table'

import { Button } from '@/components/ui/button'
import type { TDataTableFeatures } from './features'

export function SortableHeader<TData extends object, TValue>({
  column,
  title,
}: {
  column: Column<TDataTableFeatures, TData, TValue>
  title: string
}) {
  const sorted = column.getIsSorted()
  const Icon = sorted === 'asc' ? ArrowUp : sorted === 'desc' ? ArrowDown : ArrowUpDown
  return (
    <Button
      variant="ghost"
      size="sm"
      className="-ml-2 h-8"
      onClick={() => column.toggleSorting(sorted === 'asc')}
    >
      {title}
      <Icon className="size-3.5 text-muted-foreground" />
    </Button>
  )
}
