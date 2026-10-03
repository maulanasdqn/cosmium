import { Store, useStore } from '@tanstack/react-store'

import type { TFingerprintResult, TStealthResult } from '@/apis/tests'

export type TTestsState = {
  activeProfile: string | null
  fingerprint: Record<string, TFingerprintResult>
  stealth: Record<string, TStealthResult>
}

export const testsStore = new Store<TTestsState>({
  activeProfile: null,
  fingerprint: {},
  stealth: {},
})

export function setActiveProfile(profile: string) {
  testsStore.setState((state) => ({ ...state, activeProfile: profile }))
}

export function saveFingerprintResult(profile: string, result: TFingerprintResult) {
  testsStore.setState((state) => ({
    ...state,
    fingerprint: { ...state.fingerprint, [profile]: result },
  }))
}

export function saveStealthResult(profile: string, result: TStealthResult) {
  testsStore.setState((state) => ({
    ...state,
    stealth: { ...state.stealth, [profile]: result },
  }))
}

export function useActiveProfile() {
  return useStore(testsStore, (state) => state.activeProfile)
}

export function useFingerprintResult(profile: string | null) {
  return useStore(testsStore, (state) => (profile ? (state.fingerprint[profile] ?? null) : null))
}

export function useStealthResult(profile: string | null) {
  return useStore(testsStore, (state) => (profile ? (state.stealth[profile] ?? null) : null))
}
