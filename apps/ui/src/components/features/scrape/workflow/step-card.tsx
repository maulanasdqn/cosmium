import { Badge } from '@/components/ui/badge'
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card'
import { Stack } from '@/components/ui/stack'
import { Text } from '@/components/ui/typography'
import { StepActions } from './step-actions'
import { StepFields } from './step-fields'
import { stepMeta } from './step-meta'
import { stepIssues } from './schema'
import { isFollowUrls, type TStepPath, type TWorkflowStep } from './types'
import { updateStep } from './workflow-store'
import { NestedStepList } from './nested-step-list'

export function StepCard({
  step,
  path,
  position,
  total,
  label,
}: {
  step: TWorkflowStep
  path: TStepPath
  position: number
  total: number
  label: string
}) {
  const meta = stepMeta[step.type]
  const issues = stepIssues(step)
  const id = `step-${path.index}-${path.child ?? 'root'}`

  return (
    <Card size="sm">
      <CardHeader>
        <Stack direction="row" align="center" justify="between" gap="sm">
          <Stack direction="row" align="center" gap="sm">
            <Badge variant="outline">{label}</Badge>
            <CardTitle>
              <Stack direction="row" align="center" gap="sm">
                <meta.icon className="size-4 text-muted-foreground" />
                {meta.label}
              </Stack>
            </CardTitle>
          </Stack>
          <StepActions path={path} position={position} total={total} />
        </Stack>
      </CardHeader>
      <CardContent>
        <Stack gap="md">
          <StepFields
            id={id}
            step={step}
            issues={issues}
            onPatch={(patch) => updateStep(path, patch)}
          />
          {isFollowUrls(step) ? (
            <Stack gap="sm">
              <Text variant="muted">Steps to run on each linked page</Text>
              <NestedStepList parentIndex={path.index} steps={step.workflow} />
              {issues.workflow ? <Text variant="error">{issues.workflow}</Text> : null}
            </Stack>
          ) : null}
        </Stack>
      </CardContent>
    </Card>
  )
}
