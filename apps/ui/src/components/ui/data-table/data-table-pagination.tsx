import { ChevronLeft, ChevronRight } from 'lucide-react'

import { Button } from '@/components/ui/button'

export function DataTablePagination({
  pageIndex,
  pageCount,
  total,
  canPrevious,
  canNext,
  onPrevious,
  onNext,
}: {
  pageIndex: number
  pageCount: number
  total: number
  canPrevious: boolean
  canNext: boolean
  onPrevious: () => void
  onNext: () => void
}) {
  return (
    <div data-slot="data-table-pagination" className="flex items-center justify-between gap-2">
      <p className="text-xs text-muted-foreground">
        {total} {total === 1 ? 'row' : 'rows'}
      </p>
      <div className="flex items-center gap-2">
        <p className="text-xs text-muted-foreground">
          Page {pageCount === 0 ? 0 : pageIndex + 1} of {pageCount}
        </p>
        <Button variant="outline" size="icon-sm" disabled={!canPrevious} onClick={onPrevious}>
          <ChevronLeft />
        </Button>
        <Button variant="outline" size="icon-sm" disabled={!canNext} onClick={onNext}>
          <ChevronRight />
        </Button>
      </div>
    </div>
  )
}
