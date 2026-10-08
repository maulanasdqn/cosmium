import { useState } from 'react'
import { toast } from 'sonner'

import { useSaveProfile, useValidateDraft, type TDiagnostic, type TProfile } from '@/apis/profiles'
import { toErrorMessage } from '@/libs/http'

export type TPendingSave = {
  profile: TProfile
  errors: TDiagnostic[]
}

export function useProfileSave(name: string, onSaved?: (profile: TProfile) => void) {
  const validate = useValidateDraft()
  const save = useSaveProfile()
  const [pending, setPending] = useState<TPendingSave | null>(null)

  async function commit(profile: TProfile) {
    try {
      await save.mutateAsync({ name, profile })
      toast.success(`Saved ${name}`)
      setPending(null)
      onSaved?.(profile)
    } catch (error) {
      toast.error(toErrorMessage(error))
    }
  }

  async function requestSave(profile: TProfile) {
    let diagnostics: TDiagnostic[] = []
    try {
      diagnostics = await validate.mutateAsync(profile)
    } catch (error) {
      toast.error(toErrorMessage(error))
      return
    }
    const errors = diagnostics.filter((entry) => entry.severity === 'error')
    if (errors.length > 0) {
      setPending({ profile, errors })
      return
    }
    await commit(profile)
  }

  return {
    requestSave,
    pending,
    dismiss: () => setPending(null),
    saveAnyway: () => {
      if (pending) {
        void commit(pending.profile)
      }
    },
    busy: validate.isPending || save.isPending,
  }
}
