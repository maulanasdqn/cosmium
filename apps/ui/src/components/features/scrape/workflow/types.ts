export const leafStepTypes = ['click', 'input', 'scroll', 'delay', 'extract', 'script'] as const

export const stepTypes = [...leafStepTypes, 'follow_urls'] as const

export type TLeafStepType = (typeof leafStepTypes)[number]
export type TStepType = (typeof stepTypes)[number]

export type TClickStep = { type: 'click'; selector: string }
export type TInputStep = { type: 'input'; selector: string; text: string }
export type TScrollStep = {
  type: 'scroll'
  infinite: boolean
  selector: string | null
  times: number
}
export type TDelayStep = { type: 'delay'; duration_ms: number }
export type TExtractStep = {
  type: 'extract'
  name: string
  selector: string
  attribute: string | null
  limit: number
}
export type TScriptStep = {
  type: 'script'
  name: string
  code: string
  timeout_seconds: number
}

export type TLeafStep =
  TClickStep | TInputStep | TScrollStep | TDelayStep | TExtractStep | TScriptStep

export type TFollowUrlsStep = {
  type: 'follow_urls'
  name: string
  selector: string
  attribute: string | null
  limit: number
  workflow: TLeafStep[]
}

export type TWorkflowStep = TLeafStep | TFollowUrlsStep

export type TStepPath = { index: number; child?: number }

export function isFollowUrls(step: TWorkflowStep): step is TFollowUrlsStep {
  return step.type === 'follow_urls'
}

export function isLeafStepType(type: TStepType): type is TLeafStepType {
  return type !== 'follow_urls'
}
