import { withForm, type TSelectOption } from '@/components/ui/form'
import { scrapeFormDefaults } from './schema'
import { SectionCard } from './section-card'

export const TargetSection = withForm({
  defaultValues: scrapeFormDefaults,
  props: {
    profileOptions: [] as TSelectOption[],
    profilesLoading: false,
  },
  render: function Render({ form, profileOptions, profilesLoading }) {
    return (
      <SectionCard title="Target" description="The page to load and the identity to load it with.">
        <form.AppField name="url">
          {(field) => (
            <field.TextField
              label="URL"
              type="url"
              placeholder="https://example.com/products"
              autoComplete="url"
            />
          )}
        </form.AppField>
        <form.AppField name="profile">
          {(field) => (
            <field.SelectField
              label="Fingerprint profile"
              options={profileOptions}
              disabled={profilesLoading}
              placeholder={profilesLoading ? 'Loading profiles…' : 'Select a profile'}
            />
          )}
        </form.AppField>
      </SectionCard>
    )
  },
})
