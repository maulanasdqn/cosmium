import { z } from 'zod'

import type { TProxyRotation, TScrapePayload } from '@/apis/scrape'

function isUrl(value: string, protocols?: string[]): boolean {
  try {
    const url = new URL(value.trim())
    return protocols ? protocols.includes(url.protocol) : true
  } catch {
    return false
  }
}

const optionalUrl = (message: string) =>
  z.string().refine((value) => value.trim() === '' || isUrl(value), { message })

export const scrapeFormSchema = z.object({
  url: z
    .string()
    .trim()
    .min(1, 'Enter the page URL')
    .refine((value) => isUrl(value, ['http:', 'https:']), 'Use an http:// or https:// URL'),
  profile: z.string().min(1, 'Choose a profile'),
  extract: z.array(z.string()),
  script: z.string(),
  wait_for_api: z.string(),
  wait_ms: z
    .number({ error: 'Enter a number' })
    .int('Use whole milliseconds')
    .min(0, 'Must be 0 or more')
    .max(60_000, 'At most 60000 ms'),
  proxy: optionalUrl('Use a full proxy URL such as http://user:pass@host:port'),
  proxies: z.array(z.string()),
  proxy_rotation: z.enum(['round-robin', 'random']),
  retries: z
    .number({ error: 'Enter a number' })
    .int('Use a whole number')
    .min(0, 'Must be 0 or more')
    .max(5, 'At most 5 retries'),
  geo_sync: z.boolean(),
  screenshot: z.boolean(),
  include_html: z.boolean(),
  headful: z.boolean(),
})

export type TScrapeFormValues = z.infer<typeof scrapeFormSchema>

export const scrapeFormDefaults: TScrapeFormValues = {
  url: '',
  profile: '',
  extract: [],
  script: '',
  wait_for_api: '',
  wait_ms: 0,
  proxy: '',
  proxies: [],
  proxy_rotation: 'round-robin',
  retries: 0,
  geo_sync: false,
  screenshot: true,
  include_html: false,
  headful: false,
}

export const rotationOptions: { value: TProxyRotation; label: string }[] = [
  { value: 'round-robin', label: 'Round robin' },
  { value: 'random', label: 'Random' },
]

function cleanLines(lines: string[]): string[] {
  return lines.map((line) => line.trim()).filter((line) => line.length > 0)
}

function orNull(value: string): string | null {
  const trimmed = value.trim()
  return trimmed.length > 0 ? trimmed : null
}

export function toScrapePayload(values: TScrapeFormValues): TScrapePayload {
  const proxies = cleanLines(values.proxies)
  return {
    url: values.url.trim(),
    profile: values.profile,
    wait_ms: values.wait_ms,
    screenshot: values.screenshot,
    headful: values.headful,
    include_html: values.include_html,
    geo_sync: values.geo_sync,
    retries: values.retries,
    extract: cleanLines(values.extract),
    workflow: [],
    script: orNull(values.script),
    proxy: orNull(values.proxy),
    proxies,
    proxy_rotation: proxies.length > 0 ? values.proxy_rotation : null,
    wait_for_api: orNull(values.wait_for_api),
  }
}

export function fromScrapePayload(payload: TScrapePayload): TScrapeFormValues {
  return {
    url: payload.url,
    profile: payload.profile,
    extract: payload.extract,
    script: payload.script ?? '',
    wait_for_api: payload.wait_for_api ?? '',
    wait_ms: payload.wait_ms,
    proxy: payload.proxy ?? '',
    proxies: payload.proxies,
    proxy_rotation: payload.proxy_rotation ?? 'round-robin',
    retries: payload.retries,
    geo_sync: payload.geo_sync,
    screenshot: payload.screenshot,
    include_html: payload.include_html,
    headful: payload.headful,
  }
}
