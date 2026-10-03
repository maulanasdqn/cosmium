import { z } from 'zod'

import { Button } from '@/components/ui/button'
import { useAppForm } from '@/components/ui/form'
import { Stack } from '@/components/ui/stack'
import type { TGenerateProfilePayload } from '@/apis/profiles'
import { profileNameSchema } from './profile-name-schema'

const generateSchema = z.object({
  persona: z
    .string()
    .trim()
    .min(10, 'Describe the persona in at least 10 characters')
    .max(2000, 'Keep the persona under 2000 characters'),
  name: profileNameSchema,
})

export function GenerateProfileForm({
  llmReady,
  pending,
  onGenerate,
}: {
  llmReady: boolean
  pending: boolean
  onGenerate: (payload: TGenerateProfilePayload) => Promise<void>
}) {
  const form = useAppForm({
    defaultValues: { persona: '', name: '' },
    validators: { onChange: generateSchema },
    onSubmit: async ({ value }) => {
      await onGenerate({ persona: value.persona.trim(), name: value.name.trim() })
    },
  })

  return (
    <form.AppForm>
      <form.FormRoot>
        <form.AppField name="persona">
          {(field) => (
            <field.TextareaField
              label="Persona"
              rows={5}
              placeholder="A developer in Berlin on a 2023 MacBook Pro with a German and English locale…"
              description="Describe the device, operating system, location, and language the profile should look like."
            />
          )}
        </form.AppField>
        <form.AppField name="name">
          {(field) => (
            <field.TextField
              label="Profile name"
              placeholder="macbook_berlin_de"
              description="Letters, numbers, dashes, and underscores."
            />
          )}
        </form.AppField>
        <Stack direction="row">
          {llmReady ? (
            <form.SubmitButton pending={pending}>Generate profile</form.SubmitButton>
          ) : (
            <Button disabled>Generate profile</Button>
          )}
        </Stack>
      </form.FormRoot>
    </form.AppForm>
  )
}
