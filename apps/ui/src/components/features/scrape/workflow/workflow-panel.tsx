import { useProfileSummaries } from '@/apis/profiles'
import type { TScrapePayload } from '@/apis/scrape'
import { Alert, AlertDescription, AlertTitle } from '@/components/ui/alert'
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card'
import { Skeleton } from '@/components/ui/skeleton'
import { Stack } from '@/components/ui/stack'
import { Heading, Text } from '@/components/ui/typography'
import { useAppForm } from '@/components/ui/form'
import { isStepValid, toWorkflowPayload, workflowTargetSchema } from './schema'
import { StepList } from './step-list'
import { WorkflowToolbar } from './workflow-toolbar'
import { isFollowUrls, type TWorkflowStep } from './types'
import { setTarget, useWorkflowState } from './workflow-store'

function allValid(steps: TWorkflowStep[]): boolean {
  return (
    steps.length > 0 &&
    steps.every(
      (step) =>
        isStepValid(step) &&
        (!isFollowUrls(step) || step.workflow.every((child) => isStepValid(child))),
    )
  )
}

export function WorkflowPanel({
  initialProfile,
  pending,
  error,
  onSubmit,
}: {
  initialProfile?: string
  pending: boolean
  error: string | null
  onSubmit: (payload: TScrapePayload) => Promise<unknown>
}) {
  const summaries = useProfileSummaries()
  const state = useWorkflowState()

  const profiles = (summaries.data ?? [])
    .filter((profile) => !profile.load_error)
    .map((profile) => ({ value: profile.name, label: profile.name }))
  const known = (name?: string) => profiles.some((option) => option.value === name)
  const profile = known(initialProfile)
    ? (initialProfile ?? '')
    : known(state.profile)
      ? state.profile
      : (profiles[0]?.value ?? '')

  const form = useAppForm({
    defaultValues: { url: state.url, profile },
    validators: { onSubmit: workflowTargetSchema },
    onSubmit: ({ value }) => {
      setTarget({ url: value.url, profile: value.profile })
      return onSubmit(toWorkflowPayload(value, state.steps))
    },
  })

  if (summaries.isPending) {
    return <Skeleton className="h-64 w-full" />
  }

  return (
    <Stack gap="lg">
      {error ? (
        <Alert variant="destructive">
          <AlertTitle>The scrape failed</AlertTitle>
          <AlertDescription>{error}</AlertDescription>
        </Alert>
      ) : null}
      <form.AppForm>
        <form.FormRoot>
          <Card>
            <CardHeader>
              <CardTitle>Target</CardTitle>
              <CardDescription>The page the workflow starts on.</CardDescription>
            </CardHeader>
            <CardContent>
              <Stack gap="md">
                <form.AppField name="url">
                  {(field) => (
                    <field.TextField
                      label="URL"
                      type="url"
                      placeholder="example.com/search?q=shoes"
                    />
                  )}
                </form.AppField>
                <form.AppField name="profile">
                  {(field) => (
                    <field.SelectField
                      label="Fingerprint profile"
                      description="The device and browser the site will see."
                      options={profiles}
                      placeholder="Choose a profile"
                    />
                  )}
                </form.AppField>
              </Stack>
            </CardContent>
          </Card>

          <Stack gap="md">
            <Stack direction="row" align="center" justify="between" gap="md" wrap>
              <Stack gap="none">
                <Heading level={3}>Steps</Heading>
                <Text variant="muted">They run in order once the page loads.</Text>
              </Stack>
              <WorkflowToolbar steps={state.steps} />
            </Stack>
            <StepList steps={state.steps} />
          </Stack>

          <Stack direction="row" justify="end" align="center" gap="md">
            {state.steps.length > 0 && !allValid(state.steps) ? (
              <Text variant="muted">Fill in every highlighted field to run.</Text>
            ) : null}
            <form.SubmitButton pending={pending} disabled={!allValid(state.steps)}>
              Run workflow
            </form.SubmitButton>
          </Stack>
        </form.FormRoot>
      </form.AppForm>
    </Stack>
  )
}
