import { Grid, Stack } from '@/components/ui/stack'
import { NumberRow, SwitchRow, TextRow, TextareaRow } from './field-row'
import type { TWorkflowStep } from './types'

export type TStepFieldsProps = {
  id: string
  step: TWorkflowStep
  issues: Record<string, string>
  onPatch: (patch: Partial<TWorkflowStep>) => void
}

const SELECTOR_HINT = 'A CSS selector, such as .product-title or #results a'

export function StepFields({ id, step, issues, onPatch }: TStepFieldsProps) {
  if (step.type === 'click') {
    return (
      <TextRow
        id={`${id}-selector`}
        label="Selector"
        description={SELECTOR_HINT}
        placeholder=".load-more"
        monospace
        value={step.selector}
        error={issues.selector}
        onChange={(selector) => onPatch({ selector })}
      />
    )
  }

  if (step.type === 'input') {
    return (
      <Grid columns={2}>
        <TextRow
          id={`${id}-selector`}
          label="Selector"
          description={SELECTOR_HINT}
          placeholder="input[name=q]"
          monospace
          value={step.selector}
          error={issues.selector}
          onChange={(selector) => onPatch({ selector })}
        />
        <TextRow
          id={`${id}-text`}
          label="Text"
          description="What to type into the field"
          placeholder="running shoes"
          value={step.text}
          error={issues.text}
          onChange={(text) => onPatch({ text })}
        />
      </Grid>
    )
  }

  if (step.type === 'scroll') {
    return (
      <Stack gap="md">
        <SwitchRow
          id={`${id}-infinite`}
          label="Scroll until nothing new loads"
          description="Keep scrolling while the page keeps growing"
          value={step.infinite}
          onChange={(infinite) => onPatch({ infinite })}
        />
        <Grid columns={2}>
          {step.infinite ? null : (
            <NumberRow
              id={`${id}-times`}
              label="Times"
              description="How many screens to scroll"
              min={1}
              max={50}
              value={step.times}
              error={issues.times}
              onChange={(times) => onPatch({ times })}
            />
          )}
          <TextRow
            id={`${id}-selector`}
            label="Container selector"
            description="Optional. Leave empty to scroll the page itself."
            placeholder=".results-list"
            monospace
            value={step.selector ?? ''}
            error={issues.selector}
            onChange={(selector) => onPatch({ selector: selector.trim() || null })}
          />
        </Grid>
      </Stack>
    )
  }

  if (step.type === 'delay') {
    return (
      <NumberRow
        id={`${id}-duration`}
        label="Wait (ms)"
        description="Pause before the next step runs"
        min={0}
        max={60_000}
        value={step.duration_ms}
        error={issues.duration_ms}
        onChange={(duration_ms) => onPatch({ duration_ms })}
      />
    )
  }

  if (step.type === 'script') {
    return (
      <Stack gap="md">
        <Grid columns={2}>
          <TextRow
            id={`${id}-name`}
            label="Result name"
            description="The key this value appears under"
            placeholder="item_count"
            value={step.name}
            error={issues.name}
            onChange={(name) => onPatch({ name })}
          />
          <NumberRow
            id={`${id}-timeout`}
            label="Timeout (seconds)"
            min={1}
            max={120}
            value={step.timeout_seconds}
            error={issues.timeout_seconds}
            onChange={(timeout_seconds) => onPatch({ timeout_seconds })}
          />
        </Grid>
        <TextareaRow
          id={`${id}-code`}
          label="JavaScript"
          description="Runs in the page. The returned value is captured."
          placeholder="document.querySelectorAll('.item').length"
          rows={4}
          value={step.code}
          error={issues.code}
          onChange={(code) => onPatch({ code })}
        />
      </Stack>
    )
  }

  return (
    <Grid columns={2}>
      <TextRow
        id={`${id}-name`}
        label="Result name"
        description="The key these values appear under"
        placeholder={step.type === 'extract' ? 'titles' : 'pages'}
        value={step.name}
        error={issues.name}
        onChange={(name) => onPatch({ name })}
      />
      <TextRow
        id={`${id}-selector`}
        label="Selector"
        description={SELECTOR_HINT}
        placeholder={step.type === 'extract' ? '.title' : 'a.product-link'}
        monospace
        value={step.selector}
        error={issues.selector}
        onChange={(selector) => onPatch({ selector })}
      />
      <TextRow
        id={`${id}-attribute`}
        label="Attribute"
        description={
          step.type === 'extract'
            ? 'Optional. Empty returns the text, or try href or src.'
            : 'Which attribute holds the link. Usually href.'
        }
        placeholder={step.type === 'extract' ? 'href' : 'href'}
        monospace
        value={step.attribute ?? ''}
        error={issues.attribute}
        onChange={(attribute) => onPatch({ attribute: attribute.trim() || null })}
      />
      <NumberRow
        id={`${id}-limit`}
        label="Limit"
        description={step.type === 'extract' ? '0 means every match' : 'How many links to open'}
        min={0}
        max={step.type === 'extract' ? 1000 : 100}
        value={step.limit}
        error={issues.limit}
        onChange={(limit) => onPatch({ limit })}
      />
    </Grid>
  )
}
