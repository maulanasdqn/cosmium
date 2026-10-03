import type { TProbe } from '@/apis/tests'

export type TProbeStatus = 'pass' | 'fail' | 'error'

export type TProbeCategory = 'Consistency' | 'Surface'

export type TProbeFilter = 'all' | 'failed' | 'passed'

export type TProbeRow = TProbe & {
  status: TProbeStatus
  category: TProbeCategory
}

const CONSISTENCY_PREFIXES = ['lie_', 'worker_', 'iframe_']

function statusOf(probe: TProbe): TProbeStatus {
  if (probe.error) {
    return 'error'
  }
  return probe.passed ? 'pass' : 'fail'
}

function categoryOf(id: string): TProbeCategory {
  return CONSISTENCY_PREFIXES.some((prefix) => id.startsWith(prefix)) ? 'Consistency' : 'Surface'
}

export function toProbeRows(probes: TProbe[]): TProbeRow[] {
  return probes.map((probe) => ({
    ...probe,
    status: statusOf(probe),
    category: categoryOf(probe.id),
  }))
}

export function filterProbeRows(rows: TProbeRow[], filter: TProbeFilter): TProbeRow[] {
  if (filter === 'passed') {
    return rows.filter((row) => row.status === 'pass')
  }
  if (filter === 'failed') {
    return rows.filter((row) => row.status !== 'pass')
  }
  return rows
}

export function isProbeFilter(value: unknown): value is TProbeFilter {
  return value === 'all' || value === 'failed' || value === 'passed'
}
