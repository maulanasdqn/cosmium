import { Link } from '@tanstack/react-router'
import { Sparkles } from 'lucide-react'

import type { TResultSummary } from '@/apis/results'
import {
  formatDuration,
  formatExact,
  formatRelative,
  RunStatusBadge,
} from '@/components/features/runs'
import { Badge } from '@/components/ui/badge'
import { createDataTableHelper, SortableHeader } from '@/components/ui/data-table'
import { Stack } from '@/components/ui/stack'
import { Tooltip, TooltipContent, TooltipTrigger } from '@/components/ui/tooltip'
import { Text } from '@/components/ui/typography'
import { ResultRowActions } from './result-row-actions'

const helper = createDataTableHelper<TResultSummary>()

export const resultColumns = helper.columns([
  helper.accessor((row) => new Date(row.created_at_ms).toISOString(), {
    id: 'created',
    sortFn: 'datetime',
    header: ({ column }) => <SortableHeader column={column} title="Scraped" />,
    cell: ({ getValue }) => (
      <Tooltip>
        <TooltipTrigger render={<Text variant="muted" inline />}>
          {formatRelative(getValue())}
        </TooltipTrigger>
        <TooltipContent>{formatExact(getValue())}</TooltipContent>
      </Tooltip>
    ),
  }),
  helper.accessor((row) => `${row.title ?? ''} ${row.final_url ?? row.url ?? ''}`, {
    id: 'page',
    header: ({ column }) => <SortableHeader column={column} title="Page" />,
    cell: ({ row }) => (
      <Stack gap="none" className="max-w-md min-w-0">
        <Link to="/scrape/$id" params={{ id: row.original.id }}>
          <Text weight="medium" truncate>
            {row.original.title || row.original.final_url || row.original.url}
          </Text>
        </Link>
        <Text variant="mono" truncate className="text-muted-foreground">
          {row.original.final_url ?? row.original.url}
        </Text>
      </Stack>
    ),
  }),
  helper.accessor((row) => row.profile ?? '', {
    id: 'profile',
    header: ({ column }) => <SortableHeader column={column} title="Profile" />,
  }),
  helper.accessor((row) => (row.blocked ? 'blocked' : 'success'), {
    id: 'status',
    header: 'Status',
    cell: ({ row }) => (
      <Stack direction="row" gap="xs" align="center">
        <RunStatusBadge status={row.original.blocked ? 'blocked' : 'success'} />
        {row.original.has_ai ? (
          <Badge variant="outline">
            <Sparkles />
            AI
          </Badge>
        ) : null}
      </Stack>
    ),
  }),
  helper.accessor((row) => row.elapsed_ms ?? 0, {
    id: 'elapsed',
    header: ({ column }) => <SortableHeader column={column} title="Took" />,
    cell: ({ row }) => formatDuration(row.original.elapsed_ms),
  }),
  helper.display({
    id: 'actions',
    cell: ({ row }) => <ResultRowActions id={row.original.id} />,
  }),
])
