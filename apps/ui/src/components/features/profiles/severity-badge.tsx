import { Badge } from '@/components/ui/badge'
import type { TSeverity } from '@/apis/profiles'

const variantBySeverity = {
  error: 'destructive',
  warning: 'secondary',
  info: 'outline',
} as const

export function SeverityBadge({ severity }: { severity: TSeverity }) {
  return (
    <Badge variant={variantBySeverity[severity]} className="capitalize">
      {severity}
    </Badge>
  )
}
