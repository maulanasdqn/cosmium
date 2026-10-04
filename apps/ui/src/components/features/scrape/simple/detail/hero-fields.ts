import { asNumber, collectImages, firstOf, formatPrice, isRecord, type TRecord } from './values'

export const HERO_KEYS = {
  title: ['title', 'name', 'product_name', 'product_title', 'headline'],
  price: ['price', 'current_price', 'sale_price', 'amount'],
  rating: ['rating', 'stars', 'average_rating', 'review_rating'],
  images: ['images', 'image', 'image_url', 'thumbnail', 'photos'],
  brand: ['brand', 'manufacturer', 'seller', 'author', 'source'],
} as const

export type THero = {
  title: string | null
  price: string | null
  ratingScore: number | null
  ratingCount: number | null
  images: string[]
  brand: string | null
  usedKeys: string[]
}

const CONSUMED_SUBKEYS = new Set([
  'amount',
  'value',
  'current',
  'price',
  'currency',
  'currency_code',
  'score',
  'average',
  'stars',
  'count',
  'total',
  'reviews',
  'review_count',
])

function fullyShown(value: unknown): boolean {
  return !isRecord(value) || Object.keys(value).every((key) => CONSUMED_SUBKEYS.has(key))
}

function usedKey(record: TRecord, keys: readonly string[]): string[] {
  const key = keys.find((candidate) => record[candidate] !== undefined)
  return key && fullyShown(record[key]) ? [key] : []
}

export function readHero(record: TRecord): THero {
  const rating = firstOf(record, [...HERO_KEYS.rating])
  const ratingScore = isRecord(rating)
    ? asNumber(firstOf(rating, ['score', 'value', 'average', 'stars']))
    : asNumber(rating)
  const ratingCount = isRecord(rating)
    ? asNumber(firstOf(rating, ['count', 'total', 'reviews', 'review_count']))
    : asNumber(firstOf(record, ['review_count', 'reviews_count', 'ratings_count']))
  const title = firstOf(record, [...HERO_KEYS.title])
  const brand = firstOf(record, [...HERO_KEYS.brand])
  return {
    title: typeof title === 'string' ? title : null,
    price: formatPrice(firstOf(record, [...HERO_KEYS.price])),
    ratingScore,
    ratingCount,
    images: collectImages(firstOf(record, [...HERO_KEYS.images])),
    brand: typeof brand === 'string' ? brand : null,
    usedKeys: Object.values(HERO_KEYS).flatMap((keys) => usedKey(record, keys)),
  }
}

export function hasHero(hero: THero): boolean {
  return Boolean(hero.title || hero.price || hero.images.length > 0)
}
