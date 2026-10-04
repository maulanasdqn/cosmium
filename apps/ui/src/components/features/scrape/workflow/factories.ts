import type { TLeafStep, TStepType, TWorkflowStep } from './types'

const builders: Record<TStepType, () => TWorkflowStep> = {
  click: () => ({ type: 'click', selector: '' }),
  input: () => ({ type: 'input', selector: '', text: '' }),
  scroll: () => ({ type: 'scroll', infinite: false, selector: null, times: 3 }),
  delay: () => ({ type: 'delay', duration_ms: 1000 }),
  extract: () => ({ type: 'extract', name: '', selector: '', attribute: null, limit: 0 }),
  script: () => ({ type: 'script', name: '', code: '', timeout_seconds: 30 }),
  follow_urls: () => ({
    type: 'follow_urls',
    name: '',
    selector: '',
    attribute: 'href',
    limit: 5,
    workflow: [],
  }),
}

export function createStep(type: TStepType): TWorkflowStep {
  return builders[type]()
}

export function createLeafStep(type: Exclude<TStepType, 'follow_urls'>): TLeafStep {
  return builders[type]() as TLeafStep
}

export function cloneStep<TStep extends TWorkflowStep>(step: TStep): TStep {
  return structuredClone(step)
}
