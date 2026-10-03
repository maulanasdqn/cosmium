import { Textarea } from '@/components/ui/textarea'
import { FieldShell, useFieldState, type TFieldShellProps } from './field-shell'

export type TTextareaFieldProps = TFieldShellProps & {
  placeholder?: string
  rows?: number
  monospace?: boolean
}

export function TextareaField({ placeholder, rows = 4, monospace, ...shell }: TTextareaFieldProps) {
  const { field, invalid } = useFieldState<string>()
  return (
    <FieldShell {...shell} htmlFor={field.name} invalid={invalid} errors={field.state.meta.errors}>
      <Textarea
        id={field.name}
        name={field.name}
        rows={rows}
        value={field.state.value}
        placeholder={placeholder}
        aria-invalid={invalid}
        className={monospace ? 'font-mono text-xs' : undefined}
        onBlur={field.handleBlur}
        onChange={(event) => field.handleChange(event.target.value)}
      />
    </FieldShell>
  )
}
