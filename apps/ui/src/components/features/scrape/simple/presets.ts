import { FileText, Heading, Image, Link2, Table2, TextSearch } from 'lucide-react'
import type { LucideIcon } from 'lucide-react'

export const presetIds = ['summary', 'text', 'links', 'headings', 'images', 'tables'] as const

export type TPreset = (typeof presetIds)[number]

export type TPresetMeta = {
  id: TPreset
  label: string
  description: string
  icon: LucideIcon
}

export const presetMeta: TPresetMeta[] = [
  {
    id: 'summary',
    label: 'Summary',
    description: 'Title, description, language',
    icon: TextSearch,
  },
  { id: 'text', label: 'Page text', description: 'All readable text', icon: FileText },
  { id: 'links', label: 'Links', description: 'Every link and its label', icon: Link2 },
  { id: 'headings', label: 'Headings', description: 'H1 to H3 outline', icon: Heading },
  { id: 'images', label: 'Images', description: 'Image URLs and alt text', icon: Image },
  { id: 'tables', label: 'Tables', description: 'HTML tables as rows', icon: Table2 },
]

export function isPreset(value: string): value is TPreset {
  return (presetIds as readonly string[]).includes(value)
}

const snippets: Record<TPreset, string> = {
  summary: `out.summary = {
    title: document.title,
    description: document.querySelector('meta[name="description"]')?.content ?? '',
    language: document.documentElement.lang ?? '',
    canonical: document.querySelector('link[rel="canonical"]')?.href ?? location.href,
  }`,
  text: `out.text = (document.body?.innerText ?? '').replace(/\\n{3,}/g, '\\n\\n').trim().slice(0, 50000)`,
  links: `out.links = [...new Map([...document.querySelectorAll('a[href]')]
    .map((a) => [a.href, { text: (a.innerText || a.title || '').trim().slice(0, 200), href: a.href }])
    .filter(([href]) => href.startsWith('http'))).values()].slice(0, 1000)`,
  headings: `out.headings = [...document.querySelectorAll('h1, h2, h3')]
    .map((h) => ({ level: Number(h.tagName[1]), text: h.innerText.trim() }))
    .filter((h) => h.text).slice(0, 300)`,
  images: `out.images = [...new Map([...document.querySelectorAll('img')]
    .map((img) => [img.currentSrc || img.src, { src: img.currentSrc || img.src, alt: img.alt ?? '' }])
    .filter(([src]) => src.startsWith('http'))).values()].slice(0, 300)`,
  tables: `out.tables = [...document.querySelectorAll('table')].slice(0, 20).map((table) =>
    [...table.rows].slice(0, 200).map((row) => [...row.cells].map((cell) => cell.innerText.trim())))`,
}

export function buildPresetScript(presets: TPreset[]): string | null {
  if (presets.length === 0) {
    return null
  }
  const body = presets.map((preset) => snippets[preset]).join('\n  ')
  return `(() => {\n  const out = {}\n  ${body}\n  return JSON.stringify(out)\n})()`
}
