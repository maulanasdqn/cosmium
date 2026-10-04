import { z } from 'zod'

import type { TScrapeResult } from '@/apis/scrape'

const pageDataSchema = z.object({
  summary: z
    .object({
      title: z.string(),
      description: z.string(),
      language: z.string(),
      canonical: z.string(),
    })
    .partial()
    .optional(),
  text: z.string().optional(),
  links: z.array(z.object({ text: z.string(), href: z.string() })).optional(),
  headings: z.array(z.object({ level: z.number(), text: z.string() })).optional(),
  images: z.array(z.object({ src: z.string(), alt: z.string() })).optional(),
  tables: z.array(z.array(z.array(z.string()))).optional(),
})

export type TPageData = z.infer<typeof pageDataSchema>
export type TPageLink = NonNullable<TPageData['links']>[number]

export type TSelectorMatch = {
  selector: string
  values: string[]
}

function decode(value: unknown): unknown {
  if (typeof value !== 'string') {
    return value
  }
  try {
    return JSON.parse(value)
  } catch {
    return value
  }
}

export function readPageData(result: TScrapeResult): TPageData {
  const parsed = pageDataSchema.safeParse(decode(result.extracted.script))
  return parsed.success ? parsed.data : {}
}

export function readSelectorMatches(result: TScrapeResult): TSelectorMatch[] {
  return Object.entries(result.extracted)
    .filter(([key]) => key !== 'script')
    .map(([selector, value]) => ({
      selector,
      values: Array.isArray(value)
        ? value.map(String)
        : value === null || value === undefined
          ? []
          : [String(value)],
    }))
}

export function downloadJson(filename: string, data: unknown) {
  const blob = new Blob([JSON.stringify(data, null, 2)], { type: 'application/json' })
  const url = URL.createObjectURL(blob)
  const anchor = document.createElement('a')
  anchor.href = url
  anchor.download = filename
  anchor.click()
  URL.revokeObjectURL(url)
}

export function fileNameFor(url: string): string {
  try {
    return `${new URL(url).hostname.replace(/^www\./, '')}.json`
  } catch {
    return 'scrape.json'
  }
}
