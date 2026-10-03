import { Badge } from '@/components/ui/badge'
import type { TProfileSummary } from '@/apis/profiles'

function plural(count: number, word: string): string {
  return `${count} ${word}${count === 1 ? '' : 's'}`
}

export function ProfileStatusBadge({ summary }: { summary: TProfileSummary }) {
  if (summary.load_error) {
    return <Badge variant="destructive">Unreadable</Badge>
  }
  const errors = summary.errors ?? 0
  const warnings = summary.warnings ?? 0
  if (errors > 0) {
    return <Badge variant="destructive">{plural(errors, 'error')}</Badge>
  }
  if (warnings > 0) {
    return <Badge variant="secondary">{plural(warnings, 'warning')}</Badge>
  }
  return <Badge variant="outline">Valid</Badge>
}
