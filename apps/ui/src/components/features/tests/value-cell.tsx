import { Text } from '@/components/ui/typography'
import { Tooltip, TooltipContent, TooltipTrigger } from '@/components/ui/tooltip'

export function ValueCell({ value }: { value: string }) {
  if (!value) {
    return (
      <Text variant="muted" inline>
        —
      </Text>
    )
  }
  return (
    <Tooltip>
      <TooltipTrigger
        render={<Text variant="mono" inline truncate className="block max-w-56 cursor-default" />}
      >
        {value}
      </TooltipTrigger>
      <TooltipContent className="max-w-md break-all font-mono">{value}</TooltipContent>
    </Tooltip>
  )
}
