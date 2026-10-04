import { DescriptionItem, DescriptionList } from '@/components/ui/description-list'
import { Card, CardContent } from '@/components/ui/card'
import { Stack } from '@/components/ui/stack'
import { DetailSection } from './detail-section'
import { PrimitiveValue } from './primitive-value'
import { isEmpty, isPrimitive, labelOf, type TRecord } from './values'

export function DetailFields({
  record,
  exclude = [],
  depth = 0,
}: {
  record: TRecord
  exclude?: string[]
  depth?: number
}) {
  const entries = Object.entries(record).filter(
    ([key, value]) => !exclude.includes(key) && !isEmpty(value),
  )
  const simple = entries.filter(([, value]) => isPrimitive(value))
  const complex = entries.filter(([, value]) => !isPrimitive(value))

  const list =
    simple.length > 0 ? (
      <DescriptionList>
        {simple.map(([key, value]) => (
          <DescriptionItem key={key} label={labelOf(key)}>
            <PrimitiveValue value={isPrimitive(value) ? value : null} />
          </DescriptionItem>
        ))}
      </DescriptionList>
    ) : null

  return (
    <Stack gap={depth === 0 ? 'md' : 'lg'}>
      {list && depth === 0 ? (
        <Card>
          <CardContent>{list}</CardContent>
        </Card>
      ) : (
        list
      )}
      {complex.map(([key, value]) => (
        <DetailSection key={key} name={key} value={value} depth={depth} />
      ))}
    </Stack>
  )
}
