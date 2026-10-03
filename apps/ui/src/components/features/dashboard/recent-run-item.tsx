import { Item, ItemActions, ItemContent, ItemDescription, ItemTitle } from '@/components/ui/item'
import { Text } from '@/components/ui/typography'
import type { TRunEntry } from '@/stores/history'
import { formatDuration, formatRelative } from '@/components/features/runs'
import { RunStatusBadge } from '@/components/features/runs'

export function RecentRunItem({ run }: { run: TRunEntry }) {
  return (
    <Item variant="outline" size="sm">
      <ItemContent className="min-w-0">
        <ItemTitle className="w-full truncate">{run.payload.url}</ItemTitle>
        <ItemDescription>
          {run.payload.profile} · {formatRelative(run.startedAt)}
        </ItemDescription>
      </ItemContent>
      <ItemActions>
        <Text variant="small" inline>
          {formatDuration(run.elapsedMs)}
        </Text>
        <RunStatusBadge status={run.status} />
      </ItemActions>
    </Item>
  )
}
