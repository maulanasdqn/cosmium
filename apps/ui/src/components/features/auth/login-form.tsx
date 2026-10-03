import { useState } from 'react'
import { useNavigate } from '@tanstack/react-router'
import { CircleAlert } from 'lucide-react'
import { toast } from 'sonner'

import { useVerifyKey } from '@/apis/auth'
import { Alert, AlertDescription, AlertTitle } from '@/components/ui/alert'
import { useAppForm } from '@/components/ui/form'
import { signIn } from '@/stores/auth'
import { loginErrorMessage } from './login-error'
import { loginSchema, type TLoginValues } from './login-schema'

const defaultValues: TLoginValues = { apiKey: '' }

export function LoginForm() {
  const navigate = useNavigate()
  const verifyKey = useVerifyKey()
  const [error, setError] = useState<string | null>(null)

  const form = useAppForm({
    defaultValues,
    validators: { onChange: loginSchema },
    onSubmit: async ({ value }) => {
      const apiKey = value.apiKey.trim()
      setError(null)
      try {
        await verifyKey.mutateAsync({ apiKey })
        signIn(apiKey)
        toast.success('Signed in')
        await navigate({ to: '/' })
      } catch (cause) {
        setError(loginErrorMessage(cause))
      }
    },
  })

  return (
    <form.AppForm>
      <form.FormRoot>
        {error ? (
          <Alert variant="destructive">
            <CircleAlert />
            <AlertTitle>Sign in failed</AlertTitle>
            <AlertDescription>{error}</AlertDescription>
          </Alert>
        ) : null}
        <form.AppField name="apiKey">
          {(field) => (
            <field.TextField
              label="API key"
              type="password"
              placeholder="Paste your API key"
              autoComplete="current-password"
            />
          )}
        </form.AppField>
        <form.SubmitButton pending={verifyKey.isPending}>Sign in</form.SubmitButton>
      </form.FormRoot>
    </form.AppForm>
  )
}
