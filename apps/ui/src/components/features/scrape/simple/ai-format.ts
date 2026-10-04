export type TAiRow = Record<string, unknown>

export type TAiState =
  | { status: 'off' }
  | { status: 'pending' }
  | { status: 'error'; error: string }
  | { status: 'done'; data: unknown; model: string }

function isRow(value: unknown): value is TAiRow {
  return typeof value === 'object' && value !== null && !Array.isArray(value)
}

export function pickRows(data: unknown): TAiRow[] | null {
  const candidate = Array.isArray(data) ? data : isRow(data) ? data.items : null
  if (!Array.isArray(candidate) || candidate.length === 0 || !candidate.every(isRow)) {
    return null
  }
  return candidate
}

export function columnKeys(rows: TAiRow[]): string[] {
  const keys = new Set<string>()
  rows.slice(0, 50).forEach((row) => Object.keys(row).forEach((key) => keys.add(key)))
  return [...keys].slice(0, 12)
}

export function formatCell(value: unknown): string {
  if (value === null || value === undefined) {
    return '—'
  }
  if (typeof value === 'object') {
    return JSON.stringify(value)
  }
  return String(value)
}

export function titleCase(key: string): string {
  return key.replace(/[_-]+/g, ' ').replace(/^\w/, (char) => char.toUpperCase())
}
