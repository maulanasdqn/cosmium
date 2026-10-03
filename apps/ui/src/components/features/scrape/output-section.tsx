import { withForm } from '@/components/ui/form'
import { scrapeFormDefaults } from './schema'
import { SectionCard } from './section-card'

export const OutputSection = withForm({
  defaultValues: scrapeFormDefaults,
  render: function Render({ form }) {
    return (
      <SectionCard title="Output" description="What to bring back with the result.">
        <form.AppField name="screenshot">
          {(field) => (
            <field.SwitchField label="Screenshot" description="Capture the visible viewport." />
          )}
        </form.AppField>
        <form.AppField name="include_html">
          {(field) => (
            <field.SwitchField
              label="Include HTML"
              description="Return the full page source in the result."
            />
          )}
        </form.AppField>
        <form.AppField name="headful">
          {(field) => (
            <field.SwitchField
              label="Headful"
              description="Open a visible browser window on the server."
            />
          )}
        </form.AppField>
      </SectionCard>
    )
  },
})
