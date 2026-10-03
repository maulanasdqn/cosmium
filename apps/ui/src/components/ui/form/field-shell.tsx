import * as React from 'react'

import {
  Field,
  FieldContent,
  FieldDescription,
  FieldError,
  FieldLabel,
} from '@/components/ui/field'
import { toFieldIssues, useFieldContext } from './form-context'

export type TFieldShellProps = {
  label: React.ReactNode
  description?: React.ReactNode
  orientation?: 'vertical' | 'horizontal'
}

export function useFieldState<TValue>() {
  const field = useFieldContext<TValue>()
  const invalid = field.state.meta.isTouched && !field.state.meta.isValid
  return { field, invalid }
}

export function FieldShell({
  label,
  description,
  orientation = 'vertical',
  htmlFor,
  invalid,
  errors,
  children,
}: TFieldShellProps & {
  htmlFor: string
  invalid: boolean
  errors: unknown[]
  children: React.ReactNode
}) {
  if (orientation === 'horizontal') {
    return (
      <Field orientation="horizontal" data-invalid={invalid}>
        <FieldContent>
          <FieldLabel htmlFor={htmlFor}>{label}</FieldLabel>
          {description ? <FieldDescription>{description}</FieldDescription> : null}
        </FieldContent>
        {children}
      </Field>
    )
  }
  return (
    <Field data-invalid={invalid}>
      <FieldLabel htmlFor={htmlFor}>{label}</FieldLabel>
      {children}
      {description ? <FieldDescription>{description}</FieldDescription> : null}
      {invalid ? <FieldError errors={toFieldIssues(errors)} /> : null}
    </Field>
  )
}
