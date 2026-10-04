export type TRecord = Record<string, unknown>

const IMAGE_URL = /^https?:.+\.(?:avif|gif|jpe?g|png|webp|svg)(?:[?#].*)?$/i

export function isRecord(value: unknown): value is TRecord {
  return typeof value === 'object' && value !== null && !Array.isArray(value)
}

export function isImageUrl(value: unknown): value is string {
  return typeof value === 'string' && IMAGE_URL.test(value)
}

export function isUrl(value: unknown): value is string {
  return typeof value === 'string' && /^https?:\S+$/i.test(value)
}

export function isPrimitive(value: unknown): value is string | number | boolean | null {
  return value === null || ['string', 'number', 'boolean'].includes(typeof value)
}

export function labelOf(key: string): string {
  return key.replace(/[_-]+/g, ' ').replace(/^\w/, (char) => char.toUpperCase())
}

export function firstOf(record: TRecord, keys: string[]): unknown {
  const key = keys.find(
    (candidate) => record[candidate] !== undefined && record[candidate] !== null,
  )
  return key === undefined ? undefined : record[key]
}

export function asNumber(value: unknown): number | null {
  if (typeof value === 'number' && Number.isFinite(value)) {
    return value
  }
  if (typeof value === 'string') {
    const parsed = Number(value.replace(/[^\d.-]/g, ''))
    return value.trim() !== '' && Number.isFinite(parsed) ? parsed : null
  }
  return null
}

export function formatPrice(value: unknown): string | null {
  if (isRecord(value)) {
    const amount = asNumber(firstOf(value, ['amount', 'value', 'current', 'price']))
    const currency = firstOf(value, ['currency', 'currency_code'])
    if (amount === null) {
      return null
    }
    if (typeof currency === 'string' && /^[A-Z]{3}$/.test(currency)) {
      return new Intl.NumberFormat(undefined, { style: 'currency', currency }).format(amount)
    }
    return amount.toLocaleString()
  }
  if (typeof value === 'string') {
    return value
  }
  const amount = asNumber(value)
  return amount === null ? null : amount.toLocaleString()
}

export function collectImages(value: unknown): string[] {
  if (isImageUrl(value)) {
    return [value]
  }
  if (Array.isArray(value)) {
    return [...new Set(value.filter(isImageUrl))]
  }
  return []
}

export function isEmpty(value: unknown): boolean {
  if (value === null || value === undefined || value === '') {
    return true
  }
  if (Array.isArray(value)) {
    return value.length === 0
  }
  return isRecord(value) && Object.keys(value).length === 0
}
