import { CircleAlert, CircleCheck, CircleX, TriangleAlert } from 'lucide-react'

import type { TVerdict } from '@/apis/tests'
import { Badge } from '@/components/ui/badge'
import type { TProbeStatus } from './probe-rows'

const passClass = 'bg-emerald-500/15 text-emerald-600 dark:text-emerald-400'
const warnClass = 'bg-amber-500/15 text-amber-600 dark:text-amber-400'

export function ProbeStatusBadge({ status }: { status: TProbeStatus }) {
  if (status === 'pass') {
    return (
      <Badge className={passClass}>
        <CircleCheck />
        Pass
      </Badge>
    )
  }
  if (status === 'error') {
    return (
      <Badge className={warnClass}>
        <CircleAlert />
        Error
      </Badge>
    )
  }
  return (
    <Badge variant="destructive">
      <CircleX />
      Fail
    </Badge>
  )
}

export function VerdictBadge({ verdict }: { verdict: TVerdict }) {
  if (verdict === 'pass') {
    return (
      <Badge className={passClass}>
        <CircleCheck />
        Pass
      </Badge>
    )
  }
  if (verdict === 'warn') {
    return (
      <Badge className={warnClass}>
        <TriangleAlert />
        Warning
      </Badge>
    )
  }
  return (
    <Badge variant="destructive">
      <CircleX />
      Fail
    </Badge>
  )
}
