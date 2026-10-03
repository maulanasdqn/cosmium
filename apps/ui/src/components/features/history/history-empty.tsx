import { Link } from '@tanstack/react-router'
import { History, ScanSearch } from 'lucide-react'

import { Button } from '@/components/ui/button'
import {
  Empty,
  EmptyContent,
  EmptyDescription,
  EmptyHeader,
  EmptyMedia,
  EmptyTitle,
} from '@/components/ui/empty'

export function HistoryEmpty() {
  return (
    <Empty className="border">
      <EmptyHeader>
        <EmptyMedia variant="icon">
          <History />
        </EmptyMedia>
        <EmptyTitle>No runs yet</EmptyTitle>
        <EmptyDescription>
          Every scrape you run from this browser is saved here so you can inspect or repeat it.
        </EmptyDescription>
      </EmptyHeader>
      <EmptyContent>
        <Button nativeButton={false} render={<Link to="/scrape" />}>
          <ScanSearch />
          Run a scrape
        </Button>
      </EmptyContent>
    </Empty>
  )
}
