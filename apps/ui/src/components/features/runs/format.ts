const relativeFormat = new Intl.RelativeTimeFormat(undefined, { numeric: 'auto' })
const exactFormat = new Intl.DateTimeFormat(undefined, { dateStyle: 'medium', timeStyle: 'medium' })

const steps: { limit: number; unit: Intl.RelativeTimeFormatUnit; size: number }[] = [
  { limit: 60, unit: 'second', size: 1 },
  { limit: 3_600, unit: 'minute', size: 60 },
  { limit: 86_400, unit: 'hour', size: 3_600 },
  { limit: 604_800, unit: 'day', size: 86_400 },
  { limit: 2_629_800, unit: 'week', size: 604_800 },
  { limit: 31_557_600, unit: 'month', size: 2_629_800 },
  { limit: Number.POSITIVE_INFINITY, unit: 'year', size: 31_557_600 },
]

export function formatRelative(iso: string, now: number = Date.now()): string {
  const seconds = Math.round((new Date(iso).getTime() - now) / 1_000)
  const step = steps.find((candidate) => Math.abs(seconds) < candidate.limit) ?? steps[0]
  return relativeFormat.format(Math.round(seconds / step.size), step.unit)
}

export function formatExact(iso: string): string {
  return exactFormat.format(new Date(iso))
}

export function formatDuration(ms: number | null | undefined): string {
  if (ms === null || ms === undefined) {
    return '—'
  }
  return ms < 1_000 ? `${ms} ms` : `${(ms / 1_000).toFixed(1)} s`
}

export function formatBytes(bytes: number): string {
  if (bytes < 1_024) {
    return `${bytes} B`
  }
  if (bytes < 1_048_576) {
    return `${(bytes / 1_024).toFixed(1)} KB`
  }
  return `${(bytes / 1_048_576).toFixed(1)} MB`
}

export function percentOf(part: number, total: number): number {
  return total === 0 ? 0 : Math.round((part / total) * 100)
}
