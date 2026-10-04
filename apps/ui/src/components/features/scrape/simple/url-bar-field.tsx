import { Globe, ScanSearch } from 'lucide-react'

import { Field, FieldError } from '@/components/ui/field'
import {
  InputGroup,
  InputGroupAddon,
  InputGroupButton,
  InputGroupInput,
} from '@/components/ui/input-group'
import { Spinner } from '@/components/ui/spinner'
import { toFieldIssues, useFieldContext } from '@/components/ui/form/form-context'

export function UrlBarField({ pending }: { pending: boolean }) {
  const field = useFieldContext<string>()
  const invalid = field.state.meta.isTouched && !field.state.meta.isValid
  return (
    <Field data-invalid={invalid}>
      <InputGroup className="h-12 rounded-xl">
        <InputGroupAddon>
          <Globe />
        </InputGroupAddon>
        <InputGroupInput
          id={field.name}
          name={field.name}
          type="url"
          inputMode="url"
          autoFocus
          placeholder="Paste a web address, e.g. example.com/products"
          className="text-base"
          aria-label="Page address"
          aria-invalid={invalid}
          value={field.state.value}
          onBlur={field.handleBlur}
          onChange={(event) => field.handleChange(event.target.value)}
        />
        <InputGroupAddon align="inline-end">
          <InputGroupButton type="submit" variant="default" size="sm" disabled={pending}>
            {pending ? <Spinner /> : <ScanSearch />}
            {pending ? 'Scraping…' : 'Scrape'}
          </InputGroupButton>
        </InputGroupAddon>
      </InputGroup>
      {invalid ? <FieldError errors={toFieldIssues(field.state.meta.errors)} /> : null}
    </Field>
  )
}
