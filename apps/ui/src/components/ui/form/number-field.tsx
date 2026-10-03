import { Input } from '@/components/ui/input'
import { FieldShell, useFieldState, type TFieldShellProps } from './field-shell'

export type TNumberFieldProps = TFieldShellProps & {
  min?: number
  max?: number
  step?: number
}

export function NumberField({ min, max, step = 1, ...shell }: TNumberFieldProps) {
  const { field, invalid } = useFieldState<number>()
  return (
    <FieldShell {...shell} htmlFor={field.name} invalid={invalid} errors={field.state.meta.errors}>
      <Input
        id={field.name}
        name={field.name}
        type="number"
        inputMode="numeric"
        min={min}
        max={max}
        step={step}
        value={Number.isFinite(field.state.value) ? field.state.value : ''}
        aria-invalid={invalid}
        onBlur={field.handleBlur}
        onChange={(event) => field.handleChange(event.target.valueAsNumber)}
      />
    </FieldShell>
  )
}
