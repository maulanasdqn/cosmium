import { withForm } from '@/components/ui/form'
import { rotationOptions, scrapeFormDefaults } from './schema'
import { SectionCard } from './section-card'

export const NetworkSection = withForm({
  defaultValues: scrapeFormDefaults,
  render: function Render({ form }) {
    return (
      <SectionCard title="Network" description="Proxies, retries, and geography.">
        <form.AppField name="proxy">
          {(field) => (
            <field.TextField
              label="Proxy"
              placeholder="http://user:pass@host:port"
              description="A single proxy. Ignored when a proxy pool is set."
            />
          )}
        </form.AppField>
        <form.AppField name="proxies">
          {(field) => (
            <field.LinesField
              label="Proxy pool"
              description="One proxy URL per line. Blocked attempts rotate to the next proxy."
              placeholder={'http://host-a:8080\nhttp://host-b:8080'}
            />
          )}
        </form.AppField>
        <form.AppField name="proxy_rotation">
          {(field) => <field.SelectField label="Rotation" options={rotationOptions} />}
        </form.AppField>
        <form.AppField name="retries">
          {(field) => (
            <field.NumberField
              label="Retries"
              description="Extra attempts when the page looks blocked."
              min={0}
              max={5}
            />
          )}
        </form.AppField>
        <form.AppField name="geo_sync">
          {(field) => (
            <field.SwitchField label="Geo sync" description="Set timezone from the exit IP" />
          )}
        </form.AppField>
      </SectionCard>
    )
  },
})
