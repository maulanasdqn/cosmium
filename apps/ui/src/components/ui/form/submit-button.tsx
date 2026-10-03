import * as React from 'react'

import { Button } from '@/components/ui/button'
import { Spinner } from '@/components/ui/spinner'
import { useFormContext } from './form-context'

export function SubmitButton({
  children,
  pending = false,
  disabled = false,
}: {
  children: React.ReactNode
  pending?: boolean
  disabled?: boolean
}) {
  const form = useFormContext()
  return (
    <form.Subscribe selector={(state) => [state.canSubmit, state.isSubmitting] as const}>
      {([canSubmit, isSubmitting]) => (
        <Button type="submit" disabled={disabled || !canSubmit || isSubmitting || pending}>
          {isSubmitting || pending ? <Spinner /> : null}
          {children}
        </Button>
      )}
    </form.Subscribe>
  )
}
