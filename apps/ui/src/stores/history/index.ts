import { Store, useStore } from '@tanstack/react-store'
import type { TScrapePayload, TScrapeResult } from '@/apis/scrape'

const STORAGE_KEY = 'cosmium.history'
const MAX_ENTRIES = 200

export type TRunStatus = 'success' | 'blocked' | 'failed'

export type TRunEntry = {
  id: string
  startedAt: string
  payload: TScrapePayload
  status: TRunStatus
  httpStatus: number | null
  finalUrl: string | null
  elapsedMs: number | null
  attempts: number | null
  proxyUsed: string | null
  error: string | null
  resultId?: string | null
}

export type THistoryState = {
  runs: TRunEntry[]
}

function readRuns(): TRunEntry[] {
  try {
    const raw = localStorage.getItem(STORAGE_KEY)
    return raw ? (JSON.parse(raw) as TRunEntry[]) : []
  } catch {
    return []
  }
}

function persist(runs: TRunEntry[]) {
  try {
    localStorage.setItem(STORAGE_KEY, JSON.stringify(runs))
  } catch {
    return
  }
}

export const historyStore = new Store<THistoryState>({ runs: readRuns() })

historyStore.subscribe((state) => persist(state.runs))

export function recordRun(entry: TRunEntry) {
  historyStore.setState((state) => ({ runs: [entry, ...state.runs].slice(0, MAX_ENTRIES) }))
}

export function removeRun(id: string) {
  historyStore.setState((state) => ({ runs: state.runs.filter((run) => run.id !== id) }))
}

export function clearRuns() {
  historyStore.setState(() => ({ runs: [] }))
}

export function useRuns() {
  return useStore(historyStore, (state) => state.runs)
}

export function useRun(id: string) {
  return useStore(historyStore, (state) => state.runs.find((run) => run.id === id) ?? null)
}

function baseEntry(payload: TScrapePayload) {
  return { id: crypto.randomUUID(), startedAt: new Date().toISOString(), payload }
}

export function runFromResult(
  payload: TScrapePayload,
  result: TScrapeResult,
  resultId: string | null = null,
): TRunEntry {
  return {
    ...baseEntry(payload),
    resultId,
    status: result.blocked ? 'blocked' : 'success',
    httpStatus: result.http_status,
    finalUrl: result.final_url,
    elapsedMs: result.elapsed_ms,
    attempts: result.attempts,
    proxyUsed: result.proxy_used ?? null,
    error: null,
  }
}

export function runFromError(payload: TScrapePayload, error: string): TRunEntry {
  return {
    ...baseEntry(payload),
    status: 'failed',
    httpStatus: null,
    finalUrl: null,
    elapsedMs: null,
    attempts: null,
    proxyUsed: null,
    error,
  }
}
