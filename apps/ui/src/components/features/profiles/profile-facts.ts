import { z } from 'zod'

import type { TProfile } from '@/apis/profiles'

const optionalText = z.string().nullish()
const optionalNumber = z.number().nullish()

const factsSchema = z.object({
  chrome_version: optionalText,
  identity: z
    .object({
      user_agent: optionalText,
      navigator_platform: optionalText,
      navigator_vendor: optionalText,
      client_hints: z
        .object({
          platform: optionalText,
          platform_version: optionalText,
          architecture: optionalText,
          bitness: optionalText,
          mobile: z.boolean().nullish(),
          brands: z.array(z.object({ brand: z.string(), version: z.string() })).nullish(),
        })
        .partial()
        .nullish(),
    })
    .partial()
    .nullish(),
  locale: z
    .object({
      languages: z.array(z.string()).nullish(),
      accept_language: optionalText,
      timezone: optionalText,
      currency: optionalText,
    })
    .partial()
    .nullish(),
  hardware: z
    .object({
      hardware_concurrency: optionalNumber,
      device_memory_gb: optionalNumber,
      max_touch_points: optionalNumber,
    })
    .partial()
    .nullish(),
  gpu: z
    .object({ vendor: optionalText, renderer: optionalText, webgl_version: optionalText })
    .partial()
    .nullish(),
  screen: z
    .object({
      width: optionalNumber,
      height: optionalNumber,
      avail_width: optionalNumber,
      avail_height: optionalNumber,
      color_depth: optionalNumber,
      device_pixel_ratio: optionalNumber,
    })
    .partial()
    .nullish(),
  fonts: z
    .object({ installed: z.array(z.string()).nullish() })
    .partial()
    .nullish(),
  voices: z
    .array(z.object({ name: z.string(), lang: optionalText, default: z.boolean().nullish() }))
    .nullish(),
})

export type TProfileFacts = z.infer<typeof factsSchema>

export function readProfileFacts(profile: TProfile): TProfileFacts {
  const parsed = factsSchema.safeParse(profile)
  return parsed.success ? parsed.data : {}
}

export type TFact = {
  label: string
  value: string
  mono?: boolean
}

function text(value: string | number | boolean | null | undefined): string {
  if (value === null || value === undefined || value === '') {
    return '—'
  }
  if (typeof value === 'boolean') {
    return value ? 'Yes' : 'No'
  }
  return String(value)
}

function size(width?: number | null, height?: number | null): string {
  return width && height ? `${width} × ${height}` : '—'
}

export function identityFacts(facts: TProfileFacts): TFact[] {
  const identity = facts.identity
  return [
    { label: 'Navigator platform', value: text(identity?.navigator_platform) },
    { label: 'Vendor', value: text(identity?.navigator_vendor) },
    { label: 'Chrome version', value: text(facts.chrome_version) },
    { label: 'User agent', value: text(identity?.user_agent), mono: true },
  ]
}

export function clientHintFacts(facts: TProfileFacts): TFact[] {
  const hints = facts.identity?.client_hints
  const brands = hints?.brands?.map((brand) => `${brand.brand} ${brand.version}`).join(', ')
  return [
    { label: 'Platform', value: text(hints?.platform) },
    { label: 'Platform version', value: text(hints?.platform_version) },
    { label: 'Architecture', value: `${text(hints?.architecture)} / ${text(hints?.bitness)}-bit` },
    { label: 'Mobile', value: text(hints?.mobile) },
    { label: 'Brands', value: text(brands), mono: true },
  ]
}

export function hardwareFacts(facts: TProfileFacts): TFact[] {
  const hardware = facts.hardware
  return [
    { label: 'CPU cores', value: text(hardware?.hardware_concurrency) },
    {
      label: 'Device memory',
      value: hardware?.device_memory_gb ? `${hardware.device_memory_gb} GB` : '—',
    },
    { label: 'Touch points', value: text(hardware?.max_touch_points) },
  ]
}

export function gpuFacts(facts: TProfileFacts): TFact[] {
  return [
    { label: 'Vendor', value: text(facts.gpu?.vendor) },
    { label: 'Renderer', value: text(facts.gpu?.renderer), mono: true },
    { label: 'WebGL', value: text(facts.gpu?.webgl_version) },
  ]
}

export function screenFacts(facts: TProfileFacts): TFact[] {
  const screen = facts.screen
  return [
    { label: 'Resolution', value: size(screen?.width, screen?.height) },
    { label: 'Available', value: size(screen?.avail_width, screen?.avail_height) },
    { label: 'Color depth', value: screen?.color_depth ? `${screen.color_depth}-bit` : '—' },
    { label: 'Pixel ratio', value: text(screen?.device_pixel_ratio) },
  ]
}

export function localeFacts(facts: TProfileFacts): TFact[] {
  const locale = facts.locale
  return [
    { label: 'Languages', value: text(locale?.languages?.join(', ')) },
    { label: 'Accept-Language', value: text(locale?.accept_language), mono: true },
    { label: 'Timezone', value: text(locale?.timezone) },
    { label: 'Currency', value: text(locale?.currency) },
  ]
}

export function mediaFacts(facts: TProfileFacts): TFact[] {
  const voices = facts.voices ?? []
  const defaultVoice = voices.find((voice) => voice.default)?.name
  return [
    { label: 'Installed fonts', value: String(facts.fonts?.installed?.length ?? 0) },
    { label: 'Speech voices', value: String(voices.length) },
    { label: 'Default voice', value: text(defaultVoice) },
  ]
}
