import { z } from 'zod'

import type { TProfile } from '@/apis/profiles'

export const MEMORY_OPTIONS = ['0.25', '0.5', '1', '2', '4', '8'] as const

function knownTimeZones(): string[] {
  try {
    return Intl.supportedValuesOf('timeZone')
  } catch {
    return []
  }
}

const zones = knownTimeZones()

const timezoneField = z
  .string()
  .trim()
  .min(1, 'A timezone is required')
  .refine((value) => zones.length === 0 || zones.includes(value), 'Not a known IANA timezone')

const pixels = (label: string, max: number) =>
  z
    .number({ error: `${label} must be a number` })
    .int()
    .min(1, `${label} must be positive`)
    .max(max)

export const profileEditSchema = z.object({
  timezone: timezoneField,
  languages: z
    .array(z.string())
    .refine((value) => value.some((line) => line.trim() !== ''), 'Add at least one language tag'),
  accept_language: z.string().trim().min(1, 'Accept-Language is required'),
  currency: z.string().trim().max(8, 'Use a short currency code'),
  hardware_concurrency: z.number().int().min(1, 'At least 1 core').max(64, 'At most 64 cores'),
  device_memory_gb: z.enum(MEMORY_OPTIONS),
  max_touch_points: z.number().int().min(0, 'Cannot be negative').max(10, 'At most 10'),
  width: pixels('Width', 16384),
  height: pixels('Height', 16384),
  avail_width: pixels('Available width', 16384),
  avail_height: pixels('Available height', 16384),
  color_depth: z.number().int().min(1).max(48),
  device_pixel_ratio: z.number().min(0.5, 'At least 0.5').max(5, 'At most 5'),
  gpu_vendor: z.string().trim().min(1, 'A vendor is required'),
  gpu_renderer: z.string().trim().min(1, 'A renderer is required'),
  user_agent: z.string().trim().min(1, 'A user agent is required'),
  navigator_platform: z.string().trim().min(1, 'A platform is required'),
})

export type TProfileEditValues = z.infer<typeof profileEditSchema>

const sourceSchema = z.object({
  locale: z
    .object({
      timezone: z.string(),
      languages: z.array(z.string()),
      accept_language: z.string(),
      currency: z.string(),
    })
    .partial()
    .optional(),
  hardware: z
    .object({
      hardware_concurrency: z.number(),
      device_memory_gb: z.number(),
      max_touch_points: z.number(),
    })
    .partial()
    .optional(),
  screen: z
    .object({
      width: z.number(),
      height: z.number(),
      avail_width: z.number(),
      avail_height: z.number(),
      color_depth: z.number(),
      device_pixel_ratio: z.number(),
    })
    .partial()
    .optional(),
  gpu: z.object({ vendor: z.string(), renderer: z.string() }).partial().optional(),
  identity: z
    .object({ user_agent: z.string(), navigator_platform: z.string() })
    .partial()
    .optional(),
})

function nearestMemory(value: number | undefined): TProfileEditValues['device_memory_gb'] {
  if (value === undefined) {
    return '8'
  }
  return MEMORY_OPTIONS.reduce((best, option) =>
    Math.abs(Number(option) - value) < Math.abs(Number(best) - value) ? option : best,
  )
}

export function memoryWasSnapped(profile: TProfile): boolean {
  const parsed = sourceSchema.safeParse(profile)
  const stored = parsed.success ? parsed.data.hardware?.device_memory_gb : undefined
  return stored !== undefined && !MEMORY_OPTIONS.some((option) => Number(option) === stored)
}

export function readEditValues(profile: TProfile): TProfileEditValues {
  const parsed = sourceSchema.safeParse(profile)
  const source = parsed.success ? parsed.data : {}
  return {
    timezone: source.locale?.timezone ?? '',
    languages: source.locale?.languages ?? [],
    accept_language: source.locale?.accept_language ?? '',
    currency: source.locale?.currency ?? '',
    hardware_concurrency: source.hardware?.hardware_concurrency ?? 8,
    device_memory_gb: nearestMemory(source.hardware?.device_memory_gb),
    max_touch_points: source.hardware?.max_touch_points ?? 0,
    width: source.screen?.width ?? 1920,
    height: source.screen?.height ?? 1080,
    avail_width: source.screen?.avail_width ?? 1920,
    avail_height: source.screen?.avail_height ?? 1032,
    color_depth: source.screen?.color_depth ?? 24,
    device_pixel_ratio: source.screen?.device_pixel_ratio ?? 1,
    gpu_vendor: source.gpu?.vendor ?? '',
    gpu_renderer: source.gpu?.renderer ?? '',
    user_agent: source.identity?.user_agent ?? '',
    navigator_platform: source.identity?.navigator_platform ?? '',
  }
}

function section(profile: TProfile, key: string): Record<string, unknown> {
  const value = profile[key]
  return typeof value === 'object' && value !== null && !Array.isArray(value)
    ? (value as Record<string, unknown>)
    : {}
}

export function mergeEditValues(profile: TProfile, values: TProfileEditValues): TProfile {
  return {
    ...profile,
    locale: {
      ...section(profile, 'locale'),
      timezone: values.timezone.trim(),
      languages: values.languages.map((line) => line.trim()).filter((line) => line !== ''),
      accept_language: values.accept_language.trim(),
      currency: values.currency.trim(),
    },
    hardware: {
      ...section(profile, 'hardware'),
      hardware_concurrency: values.hardware_concurrency,
      device_memory_gb: Number(values.device_memory_gb),
      max_touch_points: values.max_touch_points,
    },
    screen: {
      ...section(profile, 'screen'),
      width: values.width,
      height: values.height,
      avail_width: values.avail_width,
      avail_height: values.avail_height,
      color_depth: values.color_depth,
      device_pixel_ratio: values.device_pixel_ratio,
    },
    gpu: {
      ...section(profile, 'gpu'),
      vendor: values.gpu_vendor.trim(),
      renderer: values.gpu_renderer.trim(),
    },
    identity: {
      ...section(profile, 'identity'),
      user_agent: values.user_agent.trim(),
      navigator_platform: values.navigator_platform.trim(),
    },
  }
}
