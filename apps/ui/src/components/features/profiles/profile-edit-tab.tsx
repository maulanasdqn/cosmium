import { useEffect, useMemo } from 'react'
import { RotateCcw } from 'lucide-react'

import { useAppForm } from '@/components/ui/form'
import { Button } from '@/components/ui/button'
import { Stack } from '@/components/ui/stack'
import { Text } from '@/components/ui/typography'
import type { TProfile } from '@/apis/profiles'
import { HardwareSection, IdentitySection, LocaleSection, ScreenSection } from './edit-sections'
import { IncoherentSaveDialog } from './incoherent-save-dialog'
import {
  memoryWasSnapped,
  mergeEditValues,
  profileEditSchema,
  readEditValues,
} from './profile-edit-schema'
import { useProfileSave } from './use-profile-save'

export function ProfileEditTab({
  name,
  profile,
  onDirtyChange,
}: {
  name: string
  profile: TProfile
  onDirtyChange?: (dirty: boolean) => void
}) {
  const defaultValues = useMemo(() => readEditValues(profile), [profile])
  const saver = useProfileSave(name)
  const form = useAppForm({
    defaultValues,
    validators: { onChange: profileEditSchema, onSubmit: profileEditSchema },
    onSubmit: ({ value }) => saver.requestSave(mergeEditValues(profile, value)),
  })

  return (
    <form.AppForm>
      <form.FormRoot>
        <form.Subscribe selector={(state) => state.isDirty}>
          {(dirty) => <DirtyBeacon dirty={dirty} onDirtyChange={onDirtyChange} />}
        </form.Subscribe>
        <LocaleSection form={form} />
        <HardwareSection form={form} memorySnapped={memoryWasSnapped(profile)} />
        <ScreenSection form={form} />
        <IdentitySection form={form} />
        <Stack direction="row" justify="between" align="center" wrap gap="sm">
          <Text variant="muted">
            Fields not shown here keep their current values. Use the JSON tab for the rest.
          </Text>
          <Stack direction="row" gap="sm">
            <form.Subscribe selector={(state) => state.isDirty}>
              {(dirty) => (
                <Button
                  type="button"
                  variant="outline"
                  disabled={!dirty}
                  onClick={() => form.reset()}
                >
                  <RotateCcw />
                  Revert
                </Button>
              )}
            </form.Subscribe>
            <form.SubmitButton pending={saver.busy}>Save changes</form.SubmitButton>
          </Stack>
        </Stack>
      </form.FormRoot>
      <IncoherentSaveDialog
        pending={saver.pending}
        busy={saver.busy}
        onCancel={saver.dismiss}
        onConfirm={saver.saveAnyway}
      />
    </form.AppForm>
  )
}

function DirtyBeacon({
  dirty,
  onDirtyChange,
}: {
  dirty: boolean
  onDirtyChange?: (dirty: boolean) => void
}) {
  useEffect(() => {
    onDirtyChange?.(dirty)
    return () => onDirtyChange?.(false)
  }, [dirty, onDirtyChange])
  return null
}
