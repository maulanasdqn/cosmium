import { Badge } from '@/components/ui/badge'
import { Stack } from '@/components/ui/stack'
import type { TDiagnostic } from '@/apis/profiles'

export function DiagnosticsSummary({ diagnostics }: { diagnostics: TDiagnostic[] }) {
  const errors = diagnostics.filter((d) => d.severity === 'error').length
  const warnings = diagnostics.filter((d) => d.severity === 'warning').length
  if (errors === 0 && warnings === 0) {
    return <Badge variant="outline">Valid</Badge>
  }
  return (
    <Stack direction="row" gap="xs" wrap>
      {errors > 0 ? (
        <Badge variant="destructive">
          {errors} {errors === 1 ? 'error' : 'errors'}
        </Badge>
      ) : null}
      {warnings > 0 ? (
        <Badge variant="secondary">
          {warnings} {warnings === 1 ? 'warning' : 'warnings'}
        </Badge>
      ) : null}
    </Stack>
  )
}
