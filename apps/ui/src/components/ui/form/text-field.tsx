import { Input } from '@/components/ui/input'
import { FieldShell, useFieldState, type TFieldShellProps } from './field-shell'

export type TTextFieldProps = TFieldShellProps & {
  placeholder?: string
  type?: 'text' | 'url' | 'password' | 'search'
  autoComplete?: string
}

export function TextField({ placeholder, type = 'text', autoComplete, ...shell }: TTextFieldProps) {
  const { field, invalid } = useFieldState<string>()
  return (
    <FieldShell {...shell} htmlFor={field.name} invalid={invalid} errors={field.state.meta.errors}>
      <Input
        id={field.name}
        name={field.name}
        type={type}
        value={field.state.value}
        placeholder={placeholder}
        autoComplete={autoComplete}
        aria-invalid={invalid}
        onBlur={field.handleBlur}
        onChange={(event) => field.handleChange(event.target.value)}
      />
    </FieldShell>
  )
}
