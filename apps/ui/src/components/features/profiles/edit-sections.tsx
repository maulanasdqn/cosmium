import { withForm } from '@/components/ui/form'
import { EditSectionCard } from './edit-section-card'
import { MEMORY_OPTIONS, readEditValues } from './profile-edit-schema'

const defaultValues = readEditValues({})

const memoryOptions = MEMORY_OPTIONS.map((value) => ({ value, label: `${value} GB` }))

export const LocaleSection = withForm({
  defaultValues,
  render: function Render({ form }) {
    return (
      <EditSectionCard title="Locale" description="Where the browser claims to be.">
        <form.AppField name="timezone">
          {(field) => <field.TextField label="Timezone" placeholder="America/Los_Angeles" />}
        </form.AppField>
        <form.AppField name="currency">
          {(field) => <field.TextField label="Currency" placeholder="USD" />}
        </form.AppField>
        <form.AppField name="languages">
          {(field) => (
            <field.LinesField
              label="Languages"
              description="One BCP 47 tag per line, most preferred first."
              placeholder="en-US"
            />
          )}
        </form.AppField>
        <form.AppField name="accept_language">
          {(field) => (
            <field.TextField
              label="Accept-Language"
              description="The header value sent with every request."
              placeholder="en-US,en;q=0.9"
            />
          )}
        </form.AppField>
      </EditSectionCard>
    )
  },
})

export const HardwareSection = withForm({
  defaultValues,
  props: { memorySnapped: false },
  render: function Render({ form, memorySnapped }) {
    return (
      <EditSectionCard title="Hardware" description="What the device reports about itself.">
        <form.AppField name="hardware_concurrency">
          {(field) => <field.NumberField label="CPU cores" min={1} max={64} />}
        </form.AppField>
        <form.AppField name="device_memory_gb">
          {(field) => (
            <field.SelectField
              label="Device memory"
              description={
                memorySnapped
                  ? 'This profile stores a value Chrome cannot report. Saving will change it.'
                  : 'Chrome only ever reports these values.'
              }
              options={memoryOptions}
            />
          )}
        </form.AppField>
        <form.AppField name="max_touch_points">
          {(field) => (
            <field.NumberField
              label="Touch points"
              description="0 for a desktop without a touchscreen."
              min={0}
              max={10}
            />
          )}
        </form.AppField>
      </EditSectionCard>
    )
  },
})

export const ScreenSection = withForm({
  defaultValues,
  render: function Render({ form }) {
    return (
      <EditSectionCard title="Screen" description="Display size the page can read.">
        <form.AppField name="width">
          {(field) => <field.NumberField label="Width" min={1} max={16384} />}
        </form.AppField>
        <form.AppField name="height">
          {(field) => <field.NumberField label="Height" min={1} max={16384} />}
        </form.AppField>
        <form.AppField name="avail_width">
          {(field) => <field.NumberField label="Available width" min={1} max={16384} />}
        </form.AppField>
        <form.AppField name="avail_height">
          {(field) => (
            <field.NumberField
              label="Available height"
              description="Screen height minus the taskbar or menu bar."
              min={1}
              max={16384}
            />
          )}
        </form.AppField>
        <form.AppField name="color_depth">
          {(field) => <field.NumberField label="Color depth" min={1} max={48} />}
        </form.AppField>
        <form.AppField name="device_pixel_ratio">
          {(field) => <field.NumberField label="Pixel ratio" min={0.5} max={5} step={0.5} />}
        </form.AppField>
      </EditSectionCard>
    )
  },
})

export const IdentitySection = withForm({
  defaultValues,
  render: function Render({ form }) {
    return (
      <EditSectionCard
        title="Identity and GPU"
        description="How the browser and graphics stack present."
      >
        <form.AppField name="navigator_platform">
          {(field) => <field.TextField label="Navigator platform" placeholder="Win32" />}
        </form.AppField>
        <form.AppField name="gpu_vendor">
          {(field) => <field.TextField label="GPU vendor" placeholder="Google Inc. (NVIDIA)" />}
        </form.AppField>
        <form.AppField name="gpu_renderer">
          {(field) => (
            <field.TextField label="GPU renderer" placeholder="ANGLE (NVIDIA, …, D3D11)" />
          )}
        </form.AppField>
        <form.AppField name="user_agent">
          {(field) => <field.TextareaField label="User agent" rows={3} monospace />}
        </form.AppField>
      </EditSectionCard>
    )
  },
})
