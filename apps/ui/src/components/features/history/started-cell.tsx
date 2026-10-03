import { Tooltip, TooltipContent, TooltipTrigger } from '@/components/ui/tooltip'
import { Text } from '@/components/ui/typography'
import { formatExact, formatRelative } from '@/components/features/runs'

export function StartedCell({ iso }: { iso: string }) {
  return (
    <Tooltip>
      <TooltipTrigger render={<Text inline variant="muted" />}>
        {formatRelative(iso)}
      </TooltipTrigger>
      <TooltipContent>{formatExact(iso)}</TooltipContent>
    </Tooltip>
  )
}
