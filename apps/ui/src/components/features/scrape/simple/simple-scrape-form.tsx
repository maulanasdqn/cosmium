import { ChevronDown } from 'lucide-react'

import type { TAiRequest } from '@/apis/results'
import type { TScrapePayload } from '@/apis/scrape'
import { Button } from '@/components/ui/button'
import { Card, CardContent } from '@/components/ui/card'
import { Collapsible, CollapsibleContent, CollapsibleTrigger } from '@/components/ui/collapsible'
import { useAppForm, type TSelectOption } from '@/components/ui/form'
import { Grid, Stack } from '@/components/ui/stack'
import { AiOptions } from './ai-options'
import { PresetPickerField } from './preset-picker-field'
import { rememberPrefs, type TScrapePrefs } from './scrape-prefs-store'
import { simpleScrapeSchema, toSimplePayload, type TSimpleScrapeInput } from './simple-schema'
import { UrlBarField } from './url-bar-field'

export function SimpleScrapeForm({
  profiles,
  prefs,
  pending,
  aiAvailable,
  onSubmit,
}: {
  profiles: TSelectOption[]
  prefs: TScrapePrefs
  pending: boolean
  aiAvailable: boolean
  onSubmit: (payload: TScrapePayload, ai: TAiRequest) => Promise<void>
}) {
  const defaultValues: TSimpleScrapeInput = {
    url: '',
    profile: prefs.profile,
    presets: prefs.presets,
    selector: '',
    aiFormat: prefs.aiFormat,
    instruction: '',
  }
  const form = useAppForm({
    defaultValues,
    validators: { onSubmit: simpleScrapeSchema },
    onSubmit: ({ value }) => {
      rememberPrefs({ profile: value.profile, presets: value.presets, aiFormat: value.aiFormat })
      return onSubmit(toSimplePayload(value), {
        enabled: aiAvailable && value.aiFormat,
        instruction: value.instruction,
      })
    },
  })

  return (
    <Card>
      <CardContent>
        <form.AppForm>
          <form.FormRoot>
            <form.AppField name="url">{() => <UrlBarField pending={pending} />}</form.AppField>
            <form.AppField name="presets">{() => <PresetPickerField />}</form.AppField>
            <AiOptions form={form} available={aiAvailable} />
            <Collapsible>
              <Stack gap="md">
                <CollapsibleTrigger
                  render={<Button variant="ghost" size="sm" className="self-start" />}
                >
                  More options
                  <ChevronDown />
                </CollapsibleTrigger>
                <CollapsibleContent>
                  <Grid columns={2}>
                    <form.AppField name="profile">
                      {(field) => (
                        <field.SelectField
                          label="Browser profile"
                          description="The device and browser the site will see."
                          options={profiles}
                          placeholder="Choose a profile"
                        />
                      )}
                    </form.AppField>
                    <form.AppField name="selector">
                      {(field) => (
                        <field.TextField
                          label="Custom CSS selector"
                          description="Optional, e.g. .price or #reviews .item"
                          placeholder=".product-title"
                        />
                      )}
                    </form.AppField>
                  </Grid>
                </CollapsibleContent>
              </Stack>
            </Collapsible>
          </form.FormRoot>
        </form.AppForm>
      </CardContent>
    </Card>
  )
}
