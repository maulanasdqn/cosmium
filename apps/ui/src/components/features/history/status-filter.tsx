import { ToggleGroup, ToggleGroupItem } from '@/components/ui/toggle-group'
import type { TRunStatus } from '@/stores/history'

export type TStatusFilter = TRunStatus | 'all'

const options: { value: TStatusFilter; label: string }[] = [
  { value: 'all', label: 'All' },
  { value: 'success', label: 'Success' },
  { value: 'blocked', label: 'Blocked' },
  { value: 'failed', label: 'Failed' },
]

function isStatusFilter(value: string | undefined): value is TStatusFilter {
  return options.some((option) => option.value === value)
}

export function StatusFilter({
  value,
  onChange,
}: {
  value: TStatusFilter
  onChange: (value: TStatusFilter) => void
}) {
  return (
    <ToggleGroup
      variant="outline"
      size="sm"
      spacing={0}
      value={[value]}
      aria-label="Filter by status"
      onValueChange={(next) => {
        const selected = next[0]
        onChange(isStatusFilter(selected) ? selected : 'all')
      }}
    >
      {options.map((option) => (
        <ToggleGroupItem key={option.value} value={option.value}>
          {option.label}
        </ToggleGroupItem>
      ))}
    </ToggleGroup>
  )
}
