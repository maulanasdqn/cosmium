import { Textarea } from '@/components/ui/textarea'
import { FieldShell, useFieldState, type TFieldShellProps } from './field-shell'

export type TLinesFieldProps = TFieldShellProps & {
  placeholder?: string
  rows?: number
}

export function LinesField({ placeholder, rows = 3, ...shell }: TLinesFieldProps) {
  const { field, invalid } = useFieldState<string[]>()
  return (
    <FieldShell {...shell} htmlFor={field.name} invalid={invalid} errors={field.state.meta.errors}>
      <Textarea
        id={field.name}
        name={field.name}
        rows={rows}
        className="font-mono text-xs"
        value={field.state.value.join('\n')}
        placeholder={placeholder}
        aria-invalid={invalid}
        onBlur={field.handleBlur}
        onChange={(event) => field.handleChange(event.target.value.split('\n'))}
      />
    </FieldShell>
  )
}
