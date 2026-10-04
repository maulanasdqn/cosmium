import { Check } from 'lucide-react'

import { Field, FieldDescription, FieldError, FieldLabel } from '@/components/ui/field'
import { ToggleGroup, ToggleGroupItem } from '@/components/ui/toggle-group'
import { Tooltip, TooltipContent, TooltipTrigger } from '@/components/ui/tooltip'
import { toFieldIssues, useFieldContext } from '@/components/ui/form/form-context'
import { isPreset, presetMeta, type TPreset } from './presets'

export function PresetPickerField() {
  const field = useFieldContext<TPreset[]>()
  const invalid = !field.state.meta.isValid
  return (
    <Field data-invalid={invalid}>
      <FieldLabel>What do you want from the page?</FieldLabel>
      <ToggleGroup
        multiple
        variant="outline"
        className="flex-wrap"
        value={field.state.value}
        onValueChange={(value) => field.handleChange(value.filter(isPreset))}
      >
        {presetMeta.map((preset) => (
          <Tooltip key={preset.id}>
            <TooltipTrigger
              render={
                <ToggleGroupItem
                  value={preset.id}
                  aria-label={preset.label}
                  className="group/chip aria-pressed:border-primary aria-pressed:bg-primary aria-pressed:text-primary-foreground"
                />
              }
            >
              <Check className="hidden group-aria-pressed/chip:block" />
              <preset.icon className="group-aria-pressed/chip:hidden" />
              {preset.label}
            </TooltipTrigger>
            <TooltipContent>{preset.description}</TooltipContent>
          </Tooltip>
        ))}
      </ToggleGroup>
      <FieldDescription>Pick one or more. A screenshot is always included.</FieldDescription>
      {invalid ? <FieldError errors={toFieldIssues(field.state.meta.errors)} /> : null}
    </Field>
  )
}
