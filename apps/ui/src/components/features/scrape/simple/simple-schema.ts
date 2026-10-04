import { z } from 'zod'

import type { TScrapePayload } from '@/apis/scrape'
import { buildPresetScript, presetIds } from './presets'

export const simpleScrapeSchema = z
  .object({
    url: z
      .string()
      .trim()
      .min(1, 'Paste the address of the page')
      .transform((value) => (/^https?:\/\//i.test(value) ? value : `https://${value}`))
      .refine((value) => URL.canParse(value), 'That does not look like a web address'),
    profile: z.string().min(1, 'Choose a browser profile'),
    presets: z.array(z.enum(presetIds)),
    selector: z.string(),
    aiFormat: z.boolean(),
    instruction: z.string().max(500, 'Keep it under 500 characters'),
  })
  .refine((values) => values.presets.length > 0 || values.selector.trim() !== '', {
    message: 'Pick at least one thing to collect',
    path: ['presets'],
  })

export type TSimpleScrapeInput = z.input<typeof simpleScrapeSchema>

export function toSimplePayload(values: TSimpleScrapeInput): TScrapePayload {
  const parsed = simpleScrapeSchema.parse(values)
  const selector = parsed.selector.trim()
  return {
    url: parsed.url,
    profile: parsed.profile,
    wait_ms: 0,
    screenshot: true,
    headful: false,
    include_html: false,
    geo_sync: false,
    retries: 1,
    extract: selector ? [selector] : [],
    script: buildPresetScript(parsed.presets),
    proxy: null,
    proxies: [],
    proxy_rotation: null,
    wait_for_api: null,
  }
}
