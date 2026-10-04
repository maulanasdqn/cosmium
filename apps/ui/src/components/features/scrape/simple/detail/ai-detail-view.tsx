import { Stack } from '@/components/ui/stack'
import { DetailFields } from './detail-fields'
import { DetailHero } from './detail-hero'
import { hasHero, readHero } from './hero-fields'
import { isRecord, type TRecord } from './values'

function primaryRecord(record: TRecord): TRecord {
  if (hasHero(readHero(record))) {
    return record
  }
  const nested = Object.entries(record).find(
    ([, value]) => isRecord(value) && hasHero(readHero(value)),
  )
  if (!nested || !isRecord(nested[1])) {
    return record
  }
  const [key, inner] = nested
  const rest = Object.fromEntries(Object.entries(record).filter(([other]) => other !== key))
  return { ...inner, ...rest }
}

export function AiDetailView({ record: raw }: { record: TRecord }) {
  const record = primaryRecord(raw)
  const hero = readHero(record)
  const showHero = hasHero(hero)
  const flagKeys = Object.keys(record).filter((key) => record[key] === true)
  return (
    <Stack gap="md">
      {showHero ? <DetailHero hero={hero} record={record} /> : null}
      <DetailFields record={record} exclude={showHero ? [...hero.usedKeys, ...flagKeys] : []} />
    </Stack>
  )
}
