import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from '@/components/ui/select'
import { FieldShell, useFieldState, type TFieldShellProps } from './field-shell'

export type TSelectOption = {
  value: string
  label: string
}

export type TSelectFieldProps = TFieldShellProps & {
  options: TSelectOption[]
  placeholder?: string
  disabled?: boolean
}

export function SelectField({ options, placeholder, disabled, ...shell }: TSelectFieldProps) {
  const { field, invalid } = useFieldState<string>()
  return (
    <FieldShell {...shell} htmlFor={field.name} invalid={invalid} errors={field.state.meta.errors}>
      <Select
        items={options}
        value={field.state.value || null}
        disabled={disabled}
        onValueChange={(value) => field.handleChange(value ?? '')}
      >
        <SelectTrigger id={field.name} className="w-full" aria-invalid={invalid}>
          <SelectValue placeholder={placeholder} />
        </SelectTrigger>
        <SelectContent>
          {options.map((option) => (
            <SelectItem key={option.value} value={option.value}>
              {option.label}
            </SelectItem>
          ))}
        </SelectContent>
      </Select>
    </FieldShell>
  )
}
