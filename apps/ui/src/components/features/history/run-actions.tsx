import { useState } from 'react'
import { useNavigate } from '@tanstack/react-router'
import { Archive, Info, MoreHorizontal, RotateCcw, Trash2 } from 'lucide-react'
import { toast } from 'sonner'

import { Button } from '@/components/ui/button'
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuSeparator,
  DropdownMenuTrigger,
} from '@/components/ui/dropdown-menu'
import { removeRun, type TRunEntry } from '@/stores/history'
import { RunDetailsDialog } from './run-details-dialog'

export function RunActions({ run }: { run: TRunEntry }) {
  const navigate = useNavigate()
  const [detailsOpen, setDetailsOpen] = useState(false)

  return (
    <>
      <DropdownMenu>
        <DropdownMenuTrigger
          render={<Button variant="ghost" size="icon-sm" aria-label="Run actions" />}
        >
          <MoreHorizontal />
        </DropdownMenuTrigger>
        <DropdownMenuContent align="end">
          {run.resultId ? (
            <DropdownMenuItem
              onClick={() =>
                void navigate({ to: '/scrape/$id', params: { id: run.resultId ?? '' } })
              }
            >
              <Archive />
              Open result
            </DropdownMenuItem>
          ) : null}
          <DropdownMenuItem
            onClick={() => void navigate({ to: '/scrape', search: { from: run.id } })}
          >
            <RotateCcw />
            Re-run
          </DropdownMenuItem>
          <DropdownMenuItem onClick={() => setDetailsOpen(true)}>
            <Info />
            Details
          </DropdownMenuItem>
          <DropdownMenuSeparator />
          <DropdownMenuItem
            variant="destructive"
            onClick={() => {
              removeRun(run.id)
              toast.success('Run removed')
            }}
          >
            <Trash2 />
            Delete
          </DropdownMenuItem>
        </DropdownMenuContent>
      </DropdownMenu>
      <RunDetailsDialog run={run} open={detailsOpen} onOpenChange={setDetailsOpen} />
    </>
  )
}
