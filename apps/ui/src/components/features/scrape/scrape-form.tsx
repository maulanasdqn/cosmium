import { useMemo } from 'react'

import { useProfileSummaries } from '@/apis/profiles'
import { useAppForm } from '@/components/ui/form'
import { Stack } from '@/components/ui/stack'
import { ExtractionSection } from './extraction-section'
import { NetworkSection } from './network-section'
import { OutputSection } from './output-section'
import { scrapeFormSchema, type TScrapeFormValues } from './schema'
import { TargetSection } from './target-section'

export function ScrapeForm({
  defaultValues,
  pending,
  onSubmit,
}: {
  defaultValues: TScrapeFormValues
  pending: boolean
  onSubmit: (values: TScrapeFormValues) => Promise<void>
}) {
  const profiles = useProfileSummaries()
  const profileOptions = useMemo(
    () =>
      (profiles.data ?? [])
        .filter((profile) => !profile.load_error)
        .map((profile) => ({ value: profile.name, label: profile.name })),
    [profiles.data],
  )
  const form = useAppForm({
    defaultValues,
    validators: { onChange: scrapeFormSchema, onSubmit: scrapeFormSchema },
    onSubmit: ({ value }) => onSubmit(value),
  })

  return (
    <form.AppForm>
      <form.FormRoot>
        <TargetSection
          form={form}
          profileOptions={profileOptions}
          profilesLoading={profiles.isPending}
        />
        <ExtractionSection form={form} />
        <NetworkSection form={form} />
        <OutputSection form={form} />
        <Stack direction="row" justify="end">
          <form.SubmitButton pending={pending}>Run scrape</form.SubmitButton>
        </Stack>
      </form.FormRoot>
    </form.AppForm>
  )
}
