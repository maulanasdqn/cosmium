import { Sparkles } from 'lucide-react'

import { withForm } from '@/components/ui/form'
import { Stack } from '@/components/ui/stack'
import { Text } from '@/components/ui/typography'
import type { TSimpleScrapeInput } from './simple-schema'

const defaultValues: TSimpleScrapeInput = {
  url: '',
  profile: '',
  presets: [],
  selector: '',
  aiFormat: true,
  instruction: '',
}

export const AiOptions = withForm({
  defaultValues,
  props: { available: false },
  render: function Render({ form, available }) {
    if (!available) {
      return (
        <Stack direction="row" align="center" gap="sm">
          <Sparkles className="size-4 text-muted-foreground" />
          <Text variant="muted">
            AI formatting is off. Start the engine with an LLM key to turn results into clean JSON.
          </Text>
        </Stack>
      )
    }
    return (
      <Stack gap="md">
        <form.AppField name="aiFormat">
          {(field) => (
            <field.SwitchField
              label="Format with AI"
              description="Automatically turn the page into clean, structured JSON."
            />
          )}
        </form.AppField>
        <form.Subscribe selector={(state) => state.values.aiFormat}>
          {(enabled) =>
            enabled ? (
              <form.AppField name="instruction">
                {(field) => (
                  <field.TextField
                    label="What should the result contain?"
                    description="Optional. Leave empty to let the AI pick the main data."
                    placeholder="e.g. product names, prices, and ratings"
                  />
                )}
              </form.AppField>
            ) : null
          }
        </form.Subscribe>
      </Stack>
    )
  },
})
