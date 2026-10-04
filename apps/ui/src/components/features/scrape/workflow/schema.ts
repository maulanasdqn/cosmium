import { z } from 'zod'

import type { TScrapePayload } from '@/apis/scrape'
import type { TWorkflowStep } from './types'

const selector = z.string().trim().min(1, 'A CSS selector is required')
const resultName = z
  .string()
  .trim()
  .min(1, 'Give the result a name')
  .max(60, 'Keep the name under 60 characters')
const attribute = z.string().nullable()

const clickSchema = z.object({ type: z.literal('click'), selector })

const inputSchema = z.object({
  type: z.literal('input'),
  selector,
  text: z.string().min(1, 'Enter the text to type'),
})

const scrollSchema = z.object({
  type: z.literal('scroll'),
  infinite: z.boolean(),
  selector: z.string().nullable(),
  times: z
    .number({ error: 'Enter a number' })
    .int('Use a whole number')
    .min(1, 'At least once')
    .max(50, 'At most 50 times'),
})

const delaySchema = z.object({
  type: z.literal('delay'),
  duration_ms: z
    .number({ error: 'Enter a number' })
    .int('Use whole milliseconds')
    .min(0, 'Must be 0 or more')
    .max(60_000, 'At most 60000 ms'),
})

const extractSchema = z.object({
  type: z.literal('extract'),
  name: resultName,
  selector,
  attribute,
  limit: z
    .number({ error: 'Enter a number' })
    .int('Use a whole number')
    .min(0, 'Must be 0 or more')
    .max(1000, 'At most 1000'),
})

const scriptSchema = z.object({
  type: z.literal('script'),
  name: resultName,
  code: z.string().trim().min(1, 'Enter the JavaScript to run'),
  timeout_seconds: z
    .number({ error: 'Enter a number' })
    .int('Use whole seconds')
    .min(1, 'At least 1 second')
    .max(120, 'At most 120 seconds'),
})

export const leafStepSchema = z.discriminatedUnion('type', [
  clickSchema,
  inputSchema,
  scrollSchema,
  delaySchema,
  extractSchema,
  scriptSchema,
])

const followUrlsSchema = z.object({
  type: z.literal('follow_urls'),
  name: resultName,
  selector,
  attribute,
  limit: z
    .number({ error: 'Enter a number' })
    .int('Use a whole number')
    .min(0, 'Must be 0 or more')
    .max(100, 'At most 100 pages'),
  workflow: z.array(leafStepSchema).min(1, 'Add at least one step to run on each page'),
})

export const workflowStepSchema = z.discriminatedUnion('type', [
  clickSchema,
  inputSchema,
  scrollSchema,
  delaySchema,
  extractSchema,
  scriptSchema,
  followUrlsSchema,
])

export const workflowSchema = z.array(workflowStepSchema).min(1, 'Add at least one step')

export const workflowTargetSchema = z.object({
  url: z
    .string()
    .trim()
    .min(1, 'Paste the address of the page')
    .transform((value) => (/^https?:/i.test(value) ? value : `https://${value}`))
    .refine((value) => URL.canParse(value), 'That does not look like a web address'),
  profile: z.string().min(1, 'Choose a browser profile'),
})

export type TWorkflowTargetInput = z.input<typeof workflowTargetSchema>

export function stepIssues(step: TWorkflowStep): Record<string, string> {
  const parsed = workflowStepSchema.safeParse(step)
  if (parsed.success) {
    return {}
  }
  const issues: Record<string, string> = {}
  for (const issue of parsed.error.issues) {
    const key = String(issue.path[0] ?? 'type')
    issues[key] ??= issue.message
  }
  return issues
}

export function isStepValid(step: TWorkflowStep): boolean {
  return workflowStepSchema.safeParse(step).success
}

export function toWorkflowPayload(
  target: TWorkflowTargetInput,
  steps: TWorkflowStep[],
): TScrapePayload {
  const parsed = workflowTargetSchema.parse(target)
  return {
    url: parsed.url,
    profile: parsed.profile,
    wait_ms: 0,
    screenshot: true,
    headful: false,
    include_html: false,
    geo_sync: false,
    retries: 1,
    extract: [],
    script: null,
    workflow: workflowSchema.parse(steps),
    proxy: null,
    proxies: [],
    proxy_rotation: null,
    wait_for_api: null,
  }
}
