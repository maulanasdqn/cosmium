import { z } from 'zod'
import { toast } from 'sonner'

import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
} from '@/components/ui/dialog'
import { Button } from '@/components/ui/button'
import { useAppForm } from '@/components/ui/form'
import { Stack } from '@/components/ui/stack'
import { useProfileSummaries, useSaveProfile, type TProfile } from '@/apis/profiles'
import { toErrorMessage } from '@/libs/http'
import { profileNameSchema } from './profile-name-schema'

export function DuplicateProfileDialog({
  name,
  profile,
  open,
  onOpenChange,
  onDuplicated,
}: {
  name: string
  profile: TProfile
  open: boolean
  onOpenChange: (open: boolean) => void
  onDuplicated: (newName: string) => void
}) {
  const summaries = useProfileSummaries()
  const save = useSaveProfile()
  const taken = (summaries.data ?? []).map((entry) => entry.name)

  const schema = z.object({
    name: profileNameSchema.refine(
      (value) => !taken.includes(value),
      'A profile with this name already exists',
    ),
  })

  const form = useAppForm({
    defaultValues: { name: `${name}-copy` },
    validators: { onChange: schema, onSubmit: schema },
    onSubmit: async ({ value }) => {
      try {
        await save.mutateAsync({ name: value.name, profile: { ...profile, name: value.name } })
        toast.success(`Created ${value.name}`)
        onOpenChange(false)
        onDuplicated(value.name)
      } catch (error) {
        toast.error(toErrorMessage(error))
      }
    },
  })

  return (
    <Dialog open={open} onOpenChange={onOpenChange}>
      <DialogContent>
        <DialogHeader>
          <DialogTitle>Duplicate {name}</DialogTitle>
          <DialogDescription>
            Saves an identical profile under a new name. The original is untouched.
          </DialogDescription>
        </DialogHeader>
        <form.AppForm>
          <form.FormRoot>
            <form.AppField name="name">
              {(field) => (
                <field.TextField
                  label="New profile name"
                  description="Letters, numbers, dashes, and underscores."
                />
              )}
            </form.AppField>
            <Stack direction="row" justify="end" gap="sm">
              <Button type="button" variant="outline" onClick={() => onOpenChange(false)}>
                Cancel
              </Button>
              <form.SubmitButton pending={save.isPending}>Duplicate</form.SubmitButton>
            </Stack>
          </form.FormRoot>
        </form.AppForm>
        <DialogFooter />
      </DialogContent>
    </Dialog>
  )
}
