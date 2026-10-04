import * as React from 'react'

import {
  Field,
  FieldContent,
  FieldDescription,
  FieldError,
  FieldLabel,
} from '@/components/ui/field'
import { Input } from '@/components/ui/input'
import { Switch } from '@/components/ui/switch'
import { Textarea } from '@/components/ui/textarea'

type TRowProps = {
  id: string
  label: React.ReactNode
  description?: React.ReactNode
  error?: string
}

function Shell({
  id,
  label,
  description,
  error,
  children,
}: TRowProps & { children: React.ReactNode }) {
  return (
    <Field data-invalid={Boolean(error)}>
      <FieldLabel htmlFor={id}>{label}</FieldLabel>
      {children}
      {description ? <FieldDescription>{description}</FieldDescription> : null}
      {error ? <FieldError errors={[{ message: error }]} /> : null}
    </Field>
  )
}

export function TextRow({
  value,
  onChange,
  placeholder,
  monospace,
  ...shell
}: TRowProps & {
  value: string
  onChange: (value: string) => void
  placeholder?: string
  monospace?: boolean
}) {
  return (
    <Shell {...shell}>
      <Input
        id={shell.id}
        value={value}
        placeholder={placeholder}
        aria-invalid={Boolean(shell.error)}
        className={monospace ? 'font-mono text-xs' : undefined}
        onChange={(event) => onChange(event.target.value)}
      />
    </Shell>
  )
}

export function NumberRow({
  value,
  onChange,
  min,
  max,
  ...shell
}: TRowProps & {
  value: number
  onChange: (value: number) => void
  min?: number
  max?: number
}) {
  return (
    <Shell {...shell}>
      <Input
        id={shell.id}
        type="number"
        inputMode="numeric"
        min={min}
        max={max}
        value={Number.isFinite(value) ? value : ''}
        aria-invalid={Boolean(shell.error)}
        onChange={(event) => onChange(event.target.valueAsNumber)}
      />
    </Shell>
  )
}

export function TextareaRow({
  value,
  onChange,
  placeholder,
  rows = 4,
  ...shell
}: TRowProps & {
  value: string
  onChange: (value: string) => void
  placeholder?: string
  rows?: number
}) {
  return (
    <Shell {...shell}>
      <Textarea
        id={shell.id}
        rows={rows}
        value={value}
        placeholder={placeholder}
        aria-invalid={Boolean(shell.error)}
        className="font-mono text-xs"
        onChange={(event) => onChange(event.target.value)}
      />
    </Shell>
  )
}

export function SwitchRow({
  value,
  onChange,
  id,
  label,
  description,
}: TRowProps & { value: boolean; onChange: (value: boolean) => void }) {
  return (
    <Field orientation="horizontal">
      <FieldContent>
        <FieldLabel htmlFor={id}>{label}</FieldLabel>
        {description ? <FieldDescription>{description}</FieldDescription> : null}
      </FieldContent>
      <Switch id={id} checked={value} onCheckedChange={onChange} />
    </Field>
  )
}
