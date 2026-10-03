import { CircleAlert, CircleCheck, ShieldAlert } from 'lucide-react'

import { Badge } from '@/components/ui/badge'
import type { TRunStatus } from '@/stores/history'

const statusMeta: Record<
  TRunStatus,
  { label: string; variant: 'secondary' | 'outline' | 'destructive'; icon: typeof CircleCheck }
> = {
  success: { label: 'Success', variant: 'secondary', icon: CircleCheck },
  blocked: { label: 'Blocked', variant: 'outline', icon: ShieldAlert },
  failed: { label: 'Failed', variant: 'destructive', icon: CircleAlert },
}

export function RunStatusBadge({ status }: { status: TRunStatus }) {
  const meta = statusMeta[status]
  return (
    <Badge variant={meta.variant}>
      <meta.icon data-icon="inline-start" />
      {meta.label}
    </Badge>
  )
}
