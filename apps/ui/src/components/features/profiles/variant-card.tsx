import { useNavigate } from '@tanstack/react-router'
import { toast } from 'sonner'
import { z } from 'zod'

import { Card, CardContent, CardHeader, CardTitle, CardAction } from '@/components/ui/card'
import { useAppForm } from '@/components/ui/form'
import { Stack } from '@/components/ui/stack'
import { Text } from '@/components/ui/typography'
import { useSaveProfile, type TProfile, type TProfileWithDiagnostics } from '@/apis/profiles'
import { toErrorMessage } from '@/libs/http'
import { DiagnosticsSummary } from './diagnostics-summary'
import { profileNameSchema } from './profile-name-schema'
import { profileDifferences } from './variant-diff'

const saveSchema = z.object({ name: profileNameSchema })

export function VariantCard({
  base,
  variant,
  suggestedName,
  index,
}: {
  base: TProfile
  variant: TProfileWithDiagnostics
  suggestedName: string
  index: number
}) {
  const save = useSaveProfile()
  const navigate = useNavigate()
  const differences = profileDifferences(base, variant.profile)
  const form = useAppForm({
    defaultValues: { name: suggestedName },
    validators: { onChange: saveSchema },
    onSubmit: async ({ value }) => {
      const name = value.name.trim()
      try {
        await save.mutateAsync({ name, profile: { ...variant.profile, name } })
        toast.success(`Saved ${name}`)
        await navigate({ to: '/profiles/$name', params: { name } })
      } catch (error) {
        toast.error(toErrorMessage(error))
      }
    },
  })

  return (
    <Card size="sm">
      <CardHeader>
        <CardTitle>Variant {index + 1}</CardTitle>
        <CardAction>
          <DiagnosticsSummary diagnostics={variant.diagnostics} />
        </CardAction>
      </CardHeader>
      <CardContent>
        <Stack gap="md">
          <Stack gap="xs">
            {differences.length === 0 ? (
              <Text variant="muted">Same key traits as the original.</Text>
            ) : (
              differences.map((difference) => (
                <Stack key={difference.label} direction="row" justify="between" gap="md">
                  <Text variant="muted" className="shrink-0">
                    {difference.label}
                  </Text>
                  <Text variant="mono" className="text-right">
                    {difference.value}
                  </Text>
                </Stack>
              ))
            )}
          </Stack>
          <form.AppForm>
            <form.FormRoot className="gap-3">
              <form.AppField name="name">
                {(field) => <field.TextField label="Save as" placeholder="profile_name" />}
              </form.AppField>
              <form.SubmitButton pending={save.isPending}>Save variant</form.SubmitButton>
            </form.FormRoot>
          </form.AppForm>
        </Stack>
      </CardContent>
    </Card>
  )
}
