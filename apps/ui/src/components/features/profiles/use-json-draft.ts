import { useEffect, useMemo, useState } from 'react'

import type { TProfile } from '@/apis/profiles'

export type TJsonDraft = {
  text: string
  setText: (next: string) => void
  parsed: TProfile | null
  parseError: string | null
  dirty: boolean
  revert: () => void
  reset: (profile: TProfile) => void
}

function format(profile: TProfile): string {
  return JSON.stringify(profile, null, 2)
}

function parse(text: string): { profile: TProfile | null; error: string | null } {
  try {
    const value: unknown = JSON.parse(text)
    if (typeof value !== 'object' || value === null || Array.isArray(value)) {
      return { profile: null, error: 'A profile must be a JSON object' }
    }
    return { profile: value as TProfile, error: null }
  } catch (error) {
    return { profile: null, error: error instanceof Error ? error.message : 'Invalid JSON' }
  }
}

export function useJsonDraft(profile: TProfile, onDirtyChange?: (dirty: boolean) => void) {
  const saved = useMemo(() => format(profile), [profile])
  const [text, setText] = useState(saved)

  useEffect(() => {
    setText(saved)
  }, [saved])

  const { profile: parsed, error: parseError } = useMemo(() => parse(text), [text])
  const dirty = text !== saved

  useEffect(() => {
    onDirtyChange?.(dirty)
    return () => onDirtyChange?.(false)
  }, [dirty, onDirtyChange])

  return {
    text,
    setText,
    parsed,
    parseError,
    dirty,
    revert: () => setText(saved),
  }
}
