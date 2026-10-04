import { useNavigate } from '@tanstack/react-router'
import { Eye, MoreHorizontal, Trash2 } from 'lucide-react'
import { toast } from 'sonner'

import { useDeleteResult } from '@/apis/results'
import { Button } from '@/components/ui/button'
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuSeparator,
  DropdownMenuTrigger,
} from '@/components/ui/dropdown-menu'
import { toErrorMessage } from '@/libs/http'

export function ResultRowActions({ id }: { id: string }) {
  const navigate = useNavigate()
  const remove = useDeleteResult()
  return (
    <DropdownMenu>
      <DropdownMenuTrigger
        render={<Button variant="ghost" size="icon-sm" aria-label="Result actions" />}
      >
        <MoreHorizontal />
      </DropdownMenuTrigger>
      <DropdownMenuContent align="end">
        <DropdownMenuItem onClick={() => void navigate({ to: '/scrape/$id', params: { id } })}>
          <Eye />
          Open
        </DropdownMenuItem>
        <DropdownMenuSeparator />
        <DropdownMenuItem
          variant="destructive"
          onClick={() =>
            remove.mutate(id, {
              onSuccess: () => toast.success('Result deleted'),
              onError: (cause) =>
                toast.error('Could not delete', { description: toErrorMessage(cause) }),
            })
          }
        >
          <Trash2 />
          Delete
        </DropdownMenuItem>
      </DropdownMenuContent>
    </DropdownMenu>
  )
}
