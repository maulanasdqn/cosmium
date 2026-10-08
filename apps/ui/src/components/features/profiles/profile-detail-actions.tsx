import { useState } from 'react'
import { Link, useNavigate } from '@tanstack/react-router'
import { Copy, FlaskConical, Sparkles, Trash2 } from 'lucide-react'
import { toast } from 'sonner'

import { Button } from '@/components/ui/button'
import { useRepairProfile, type TProfile } from '@/apis/profiles'
import { toErrorMessage } from '@/libs/http'
import { AiActionButton } from './ai-action-button'
import { DeleteProfileDialog } from './delete-profile-dialog'
import { DuplicateProfileDialog } from './duplicate-profile-dialog'
import { RepairDialog } from './repair-dialog'

export function ProfileDetailActions({
  name,
  profile,
  llmReady,
}: {
  name: string
  profile: TProfile
  llmReady: boolean
}) {
  const navigate = useNavigate()
  const repair = useRepairProfile()
  const [deleting, setDeleting] = useState(false)
  const [duplicating, setDuplicating] = useState(false)

  function startRepair() {
    repair.mutate(name, { onError: (error) => toast.error(toErrorMessage(error)) })
  }

  return (
    <>
      <Button variant="outline" render={<Link to="/tests" search={{ profile: name }} />}>
        <FlaskConical />
        Run tests
      </Button>
      <Button variant="outline" onClick={() => setDuplicating(true)}>
        <Copy />
        Duplicate
      </Button>
      <AiActionButton llmReady={llmReady} pending={repair.isPending} onClick={startRepair}>
        <Sparkles />
        Repair with AI
      </AiActionButton>
      <Button variant="destructive" onClick={() => setDeleting(true)}>
        <Trash2 />
        Delete
      </Button>
      <DuplicateProfileDialog
        name={name}
        profile={profile}
        open={duplicating}
        onOpenChange={setDuplicating}
        onDuplicated={(newName) =>
          void navigate({ to: '/profiles/$name', params: { name: newName } })
        }
      />
      <DeleteProfileDialog
        name={name}
        open={deleting}
        onOpenChange={setDeleting}
        onDeleted={() => void navigate({ to: '/profiles' })}
      />
      <RepairDialog name={name} result={repair.data ?? null} onClose={() => repair.reset()} />
    </>
  )
}
