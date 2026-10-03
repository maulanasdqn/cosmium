import { toast } from 'sonner'
import { z } from 'zod'

import { Alert, AlertDescription, AlertTitle } from '@/components/ui/alert'
import { Button } from '@/components/ui/button'
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card'
import { useAppForm } from '@/components/ui/form'
import { Grid, Stack } from '@/components/ui/stack'
import { useMutateProfile, type TProfile } from '@/apis/profiles'
import { toErrorMessage } from '@/libs/http'
import { LLM_MISSING_MESSAGE } from './ai-action-button'
import { VariantCard } from './variant-card'

const mutateSchema = z.object({
  count: z.number().int().min(1, 'At least 1').max(5, 'At most 5'),
  hint: z.string().max(500, 'Keep the hint under 500 characters'),
})

export function VariantsPanel({
  name,
  profile,
  llmReady,
}: {
  name: string
  profile: TProfile
  llmReady: boolean
}) {
  const mutate = useMutateProfile()
  const form = useAppForm({
    defaultValues: { count: 3, hint: '' },
    validators: { onChange: mutateSchema },
    onSubmit: async ({ value }) => {
      try {
        await mutate.mutateAsync({ name, count: value.count, hint: value.hint.trim() || null })
      } catch (error) {
        toast.error(toErrorMessage(error))
      }
    },
  })

  return (
    <Stack gap="lg">
      <Card>
        <CardHeader>
          <CardTitle>Generate variants</CardTitle>
          <CardDescription>
            Ask the AI for coherent siblings of this profile with small, believable differences.
          </CardDescription>
        </CardHeader>
        <CardContent>
          {llmReady ? null : (
            <Alert className="mb-4">
              <AlertTitle>AI is not configured</AlertTitle>
              <AlertDescription>{LLM_MISSING_MESSAGE}</AlertDescription>
            </Alert>
          )}
          <form.AppForm>
            <form.FormRoot className="gap-4">
              <Grid columns={2}>
                <form.AppField name="count">
                  {(field) => <field.NumberField label="Variants" min={1} max={5} />}
                </form.AppField>
                <form.AppField name="hint">
                  {(field) => (
                    <field.TextField label="Hint" placeholder="e.g. vary the GPU and screen size" />
                  )}
                </form.AppField>
              </Grid>
              <Stack direction="row">
                {llmReady ? (
                  <form.SubmitButton pending={mutate.isPending}>
                    Generate variants
                  </form.SubmitButton>
                ) : (
                  <Button disabled>Generate variants</Button>
                )}
              </Stack>
            </form.FormRoot>
          </form.AppForm>
        </CardContent>
      </Card>
      {mutate.data ? (
        <Grid columns={2}>
          {mutate.data.map((variant, index) => (
            <VariantCard
              key={`${name}-${index}`}
              base={profile}
              variant={variant}
              index={index}
              suggestedName={`${name}_v${index + 1}`}
            />
          ))}
        </Grid>
      ) : null}
    </Stack>
  )
}
