import { Link } from '@tanstack/react-router'

import { Button } from '@/components/ui/button'
import { SortableHeader, createDataTableHelper } from '@/components/ui/data-table'
import { Stack } from '@/components/ui/stack'
import { Tooltip, TooltipContent, TooltipTrigger } from '@/components/ui/tooltip'
import { Text } from '@/components/ui/typography'
import type { TProfileSummary } from '@/apis/profiles'
import { ProfileRowActions } from './profile-row-actions'
import { ProfileStatusBadge } from './profile-status-badge'

const helper = createDataTableHelper<TProfileSummary>()

function dash(value?: string | null): string {
  return value ? value : '—'
}

export const profileColumns = helper.columns([
  helper.accessor('name', {
    header: ({ column }) => <SortableHeader column={column} title="Name" />,
    cell: ({ row }) => (
      <Button
        variant="link"
        className="h-auto px-0"
        render={<Link to="/profiles/$name" params={{ name: row.original.name }} />}
      >
        {row.original.name}
      </Button>
    ),
  }),
  helper.accessor((row) => `${row.navigator_platform ?? ''} ${row.client_hints_platform ?? ''}`, {
    id: 'platform',
    header: ({ column }) => <SortableHeader column={column} title="Platform" />,
    cell: ({ row }) => (
      <Stack gap="none">
        <Text weight="medium">{dash(row.original.navigator_platform)}</Text>
        <Text variant="small">{dash(row.original.client_hints_platform)}</Text>
      </Stack>
    ),
  }),
  helper.accessor((row) => row.chrome_version ?? '', {
    id: 'chrome',
    header: ({ column }) => <SortableHeader column={column} title="Chrome" />,
    cell: ({ row }) => <Text variant="mono">{dash(row.original.chrome_version)}</Text>,
  }),
  helper.accessor((row) => row.gpu_renderer ?? '', {
    id: 'gpu',
    header: 'GPU',
    cell: ({ row }) => (
      <Tooltip>
        <TooltipTrigger render={<Text truncate className="max-w-56" />}>
          {dash(row.original.gpu_renderer)}
        </TooltipTrigger>
        <TooltipContent>{dash(row.original.gpu_renderer)}</TooltipContent>
      </Tooltip>
    ),
  }),
  helper.accessor((row) => row.timezone ?? '', {
    id: 'timezone',
    header: ({ column }) => <SortableHeader column={column} title="Timezone" />,
    cell: ({ row }) => <Text>{dash(row.original.timezone)}</Text>,
  }),
  helper.accessor((row) => row.screen ?? '', {
    id: 'screen',
    header: 'Screen',
    cell: ({ row }) => (
      <Text>
        {dash(row.original.screen)}
        {row.original.device_pixel_ratio ? ` @${row.original.device_pixel_ratio}x` : ''}
      </Text>
    ),
  }),
  helper.display({
    id: 'status',
    header: 'Status',
    cell: ({ row }) => <ProfileStatusBadge summary={row.original} />,
  }),
  helper.display({
    id: 'actions',
    header: '',
    cell: ({ row }) => <ProfileRowActions name={row.original.name} />,
  }),
])
