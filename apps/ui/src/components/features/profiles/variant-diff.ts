import type { TProfile } from '@/apis/profiles'
import { readProfileFacts, type TProfileFacts } from './profile-facts'

export type TDifference = {
  label: string
  value: string
}

type TPick = (facts: TProfileFacts) => string

const tracked: Array<[string, TPick]> = [
  ['User agent', (f) => f.identity?.user_agent ?? ''],
  ['Platform', (f) => f.identity?.navigator_platform ?? ''],
  ['GPU', (f) => f.gpu?.renderer ?? ''],
  ['Screen', (f) => (f.screen?.width ? `${f.screen.width} × ${f.screen.height ?? 0}` : '')],
  ['Timezone', (f) => f.locale?.timezone ?? ''],
  ['Languages', (f) => f.locale?.languages?.join(', ') ?? ''],
  ['CPU cores', (f) => String(f.hardware?.hardware_concurrency ?? '')],
  ['Memory', (f) => String(f.hardware?.device_memory_gb ?? '')],
]

export function profileDifferences(base: TProfile, variant: TProfile): TDifference[] {
  const before = readProfileFacts(base)
  const after = readProfileFacts(variant)
  return tracked
    .filter(([, pick]) => pick(before) !== pick(after))
    .map(([label, pick]) => ({ label, value: pick(after) || '—' }))
}

export function profileName(profile: TProfile, fallback: string): string {
  return typeof profile.name === 'string' && profile.name ? profile.name : fallback
}
