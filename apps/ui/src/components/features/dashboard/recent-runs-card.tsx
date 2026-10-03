import { Link } from '@tanstack/react-router'
import { History } from 'lucide-react'

import { Button } from '@/components/ui/button'
import {
  Card,
  CardAction,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from '@/components/ui/card'
import { Empty, EmptyDescription, EmptyHeader, EmptyMedia, EmptyTitle } from '@/components/ui/empty'
import { ItemGroup } from '@/components/ui/item'
import { useRuns } from '@/stores/history'
import { RecentRunItem } from './recent-run-item'

const RECENT_LIMIT = 5

export function RecentRunsCard() {
  const runs = useRuns().slice(0, RECENT_LIMIT)

  return (
    <Card>
      <CardHeader>
        <CardTitle>Recent runs</CardTitle>
        <CardDescription>The latest scrapes started from this browser</CardDescription>
        <CardAction>
          <Button variant="outline" size="sm" render={<Link to="/history" />}>
            View all
          </Button>
        </CardAction>
      </CardHeader>
      <CardContent>
        {runs.length === 0 ? (
          <Empty>
            <EmptyHeader>
              <EmptyMedia variant="icon">
                <History />
              </EmptyMedia>
              <EmptyTitle>No runs yet</EmptyTitle>
              <EmptyDescription>Start a scrape and it will show up here.</EmptyDescription>
            </EmptyHeader>
          </Empty>
        ) : (
          <ItemGroup>
            {runs.map((run) => (
              <RecentRunItem key={run.id} run={run} />
            ))}
          </ItemGroup>
        )}
      </CardContent>
    </Card>
  )
}
