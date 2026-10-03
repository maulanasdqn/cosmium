import { useState } from 'react'
import { Link, useNavigate } from '@tanstack/react-router'
import { FlaskConical, Sparkles, Trash2 } from 'lucide-react'
import { toast } from 'sonner'

import { Button } from '@/components/ui/button'
import { useRepairProfile } from '@/apis/profiles'
import { toErrorMessage } from '@/libs/http'
import { AiActionButton } from './ai-action-button'
import { DeleteProfileDialog } from './delete-profile-dialog'
import { RepairDialog } from './repair-dialog'

export function ProfileDetailActions({ name, llmReady }: { name: string; llmReady: boolean }) {
  const navigate = useNavigate()
  const repair = useRepairProfile()
  const [deleting, setDeleting] = useState(false)

  function startRepair() {
    repair.mutate(name, { onError: (error) => toast.error(toErrorMessage(error)) })
  }

  return (
    <>
      <Button variant="outline" render={<Link to="/tests" search={{ profile: name }} />}>
        <FlaskConical />
        Run tests
      </Button>
      <AiActionButton llmReady={llmReady} pending={repair.isPending} onClick={startRepair}>
        <Sparkles />
        Repair with AI
      </AiActionButton>
      <Button variant="destructive" onClick={() => setDeleting(true)}>
        <Trash2 />
        Delete
      </Button>
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
