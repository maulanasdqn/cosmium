import { useNavigate } from '@tanstack/react-router'
import { Save } from 'lucide-react'
import { toast } from 'sonner'

import { Button } from '@/components/ui/button'
import {
  Card,
  CardAction,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from '@/components/ui/card'
import { Spinner } from '@/components/ui/spinner'
import { Stack } from '@/components/ui/stack'
import { useSaveProfile, type TProfileWithDiagnostics } from '@/apis/profiles'
import { toErrorMessage } from '@/libs/http'
import { DiagnosticsSummary } from './diagnostics-summary'
import { DiagnosticsTable } from './diagnostics-table'
import { ProfileJson } from './profile-json'

export function GeneratedProfilePreview({
  name,
  result,
}: {
  name: string
  result: TProfileWithDiagnostics
}) {
  const save = useSaveProfile()
  const navigate = useNavigate()

  function persist() {
    save.mutate(
      { name, profile: result.profile },
      {
        onSuccess: () => {
          toast.success(`Saved ${name}`)
          void navigate({ to: '/profiles/$name', params: { name } })
        },
        onError: (error) => toast.error(toErrorMessage(error)),
      },
    )
  }

  return (
    <Card>
      <CardHeader>
        <CardTitle>{name}</CardTitle>
        <CardDescription>Review the generated profile before saving it.</CardDescription>
        <CardAction>
          <Button disabled={save.isPending} onClick={persist}>
            {save.isPending ? <Spinner /> : <Save />}
            Save profile
          </Button>
        </CardAction>
      </CardHeader>
      <CardContent>
        <Stack gap="md">
          <DiagnosticsSummary diagnostics={result.diagnostics} />
          <DiagnosticsTable diagnostics={result.diagnostics} />
          <ProfileJson profile={result.profile} maxHeight="max-h-96" />
        </Stack>
      </CardContent>
    </Card>
  )
}
