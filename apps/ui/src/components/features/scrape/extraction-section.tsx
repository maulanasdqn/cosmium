import { withForm } from '@/components/ui/form'
import { scrapeFormDefaults } from './schema'
import { SectionCard } from './section-card'

export const ExtractionSection = withForm({
  defaultValues: scrapeFormDefaults,
  render: function Render({ form }) {
    return (
      <SectionCard title="Extraction" description="What to pull out of the page once it settles.">
        <form.AppField name="extract">
          {(field) => (
            <field.LinesField
              label="CSS selectors"
              description="One selector per line. The text of each match is returned."
              placeholder={'h1\n.price\n#reviews .item'}
            />
          )}
        </form.AppField>
        <form.AppField name="script">
          {(field) => (
            <field.TextareaField
              label="Script"
              description="Optional JavaScript evaluated in the page. Its return value is captured."
              placeholder="document.title"
              monospace
            />
          )}
        </form.AppField>
        <form.AppField name="wait_for_api">
          {(field) => (
            <field.TextField
              label="Wait for API"
              description="Wait for a response whose URL contains this text and capture its body."
              placeholder="/api/search"
            />
          )}
        </form.AppField>
        <form.AppField name="wait_ms">
          {(field) => (
            <field.NumberField
              label="Extra wait (ms)"
              description="Additional settle time after the page loads."
              min={0}
              max={60_000}
              step={250}
            />
          )}
        </form.AppField>
      </SectionCard>
    )
  },
})
