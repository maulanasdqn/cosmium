import { createFormHookContexts } from '@tanstack/react-form'

export const { fieldContext, formContext, useFieldContext, useFormContext } =
  createFormHookContexts()

export type TFieldIssue = { message?: string }

export function toFieldIssues(errors: unknown[]): TFieldIssue[] {
  return errors.map((error) => {
    if (typeof error === 'string') {
      return { message: error }
    }
    if (error && typeof error === 'object' && 'message' in error) {
      return { message: String((error as { message: unknown }).message) }
    }
    return { message: String(error) }
  })
}
