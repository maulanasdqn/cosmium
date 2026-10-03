import { Switch } from '@/components/ui/switch'
import { FieldShell, useFieldState, type TFieldShellProps } from './field-shell'

export function SwitchField(shell: Omit<TFieldShellProps, 'orientation'>) {
  const { field, invalid } = useFieldState<boolean>()
  return (
    <FieldShell
      {...shell}
      orientation="horizontal"
      htmlFor={field.name}
      invalid={invalid}
      errors={field.state.meta.errors}
    >
      <Switch
        id={field.name}
        name={field.name}
        checked={field.state.value}
        onCheckedChange={(checked) => field.handleChange(checked)}
      />
    </FieldShell>
  )
}
