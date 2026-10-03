import { FlaskConical, ShieldCheck } from 'lucide-react'

import { Button } from '@/components/ui/button'
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card'
import { useAppForm, type TSelectOption } from '@/components/ui/form'
import { Spinner } from '@/components/ui/spinner'
import { Grid, Stack } from '@/components/ui/stack'
import {
  testsFormSchema,
  type TTestAction,
  type TTestsFormValues,
  type TTestsSubmitMeta,
} from './tests-schema'

export type TTestsConfigCardProps = {
  defaultProfile?: string
  profileOptions: TSelectOption[]
  profilesLoading: boolean
  pendingAction: TTestAction | null
  onRun: (action: TTestAction, values: TTestsFormValues) => void
}

const defaultMeta: TTestsSubmitMeta = { action: 'fingerprint' }

export function TestsConfigCard({
  defaultProfile,
  profileOptions,
  profilesLoading,
  pendingAction,
  onRun,
}: TTestsConfigCardProps) {
  const defaultValues: TTestsFormValues = {
    profile: defaultProfile ?? '',
    geo_sync: false,
    bot_check_url: '',
  }
  const form = useAppForm({
    defaultValues,
    validators: { onChange: testsFormSchema },
    onSubmitMeta: defaultMeta,
    onSubmit: ({ value, meta }) => onRun(meta.action, value),
  })
  const busy = pendingAction !== null

  return (
    <Card>
      <CardHeader>
        <CardTitle>Test configuration</CardTitle>
        <CardDescription>
          Launch the browser with a profile and check what websites can observe.
        </CardDescription>
      </CardHeader>
      <CardContent>
        <form.AppForm>
          <form.FormRoot>
            <Grid columns={2}>
              <form.AppField name="profile">
                {(field) => (
                  <field.SelectField
                    label="Profile"
                    options={profileOptions}
                    disabled={profilesLoading}
                    placeholder={profilesLoading ? 'Loading profiles…' : 'Select a profile'}
                  />
                )}
              </form.AppField>
              <form.AppField name="bot_check_url">
                {(field) => (
                  <field.TextField
                    label="Bot-check URL"
                    type="url"
                    placeholder="https://example.com"
                    description="Optional. Adds a page of your own to the external checks."
                  />
                )}
              </form.AppField>
            </Grid>
            <form.AppField name="geo_sync">
              {(field) => (
                <field.SwitchField
                  label="Set timezone from the exit IP"
                  description="Looks up the exit IP's timezone before launching the browser."
                />
              )}
            </form.AppField>
            <Stack direction="responsive" gap="sm" justify="end">
              <Button
                type="submit"
                disabled={busy}
                onClick={(event) => {
                  event.preventDefault()
                  void form.handleSubmit({ action: 'fingerprint' })
                }}
              >
                {pendingAction === 'fingerprint' ? <Spinner /> : <FlaskConical />}
                Run fingerprint probes
              </Button>
              <Button
                type="button"
                variant="outline"
                disabled={busy}
                onClick={() => void form.handleSubmit({ action: 'stealth' })}
              >
                {pendingAction === 'stealth' ? <Spinner /> : <ShieldCheck />}
                Run external checks
              </Button>
            </Stack>
          </form.FormRoot>
        </form.AppForm>
      </CardContent>
    </Card>
  )
}
