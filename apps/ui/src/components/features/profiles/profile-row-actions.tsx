import { useState } from 'react'
import { useNavigate } from '@tanstack/react-router'
import { Eye, FlaskConical, MoreHorizontal, ScanSearch, Trash2 } from 'lucide-react'

import { Button } from '@/components/ui/button'
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuSeparator,
  DropdownMenuTrigger,
} from '@/components/ui/dropdown-menu'
import { DeleteProfileDialog } from './delete-profile-dialog'

export function ProfileRowActions({ name }: { name: string }) {
  const navigate = useNavigate()
  const [deleting, setDeleting] = useState(false)

  return (
    <>
      <DropdownMenu>
        <DropdownMenuTrigger
          render={<Button variant="ghost" size="icon-sm" aria-label={`Actions for ${name}`} />}
          onClick={(event) => event.stopPropagation()}
        >
          <MoreHorizontal />
        </DropdownMenuTrigger>
        <DropdownMenuContent align="end" onClick={(event) => event.stopPropagation()}>
          <DropdownMenuItem
            onClick={() => void navigate({ to: '/profiles/$name', params: { name } })}
          >
            <Eye />
            View
          </DropdownMenuItem>
          <DropdownMenuItem
            onClick={() => void navigate({ to: '/tests', search: { profile: name } })}
          >
            <FlaskConical />
            Run stealth tests
          </DropdownMenuItem>
          <DropdownMenuItem
            onClick={() => void navigate({ to: '/scrape', search: { profile: name } })}
          >
            <ScanSearch />
            Scrape
          </DropdownMenuItem>
          <DropdownMenuSeparator />
          <DropdownMenuItem variant="destructive" onClick={() => setDeleting(true)}>
            <Trash2 />
            Delete
          </DropdownMenuItem>
        </DropdownMenuContent>
      </DropdownMenu>
      <DeleteProfileDialog name={name} open={deleting} onOpenChange={setDeleting} />
    </>
  )
}
