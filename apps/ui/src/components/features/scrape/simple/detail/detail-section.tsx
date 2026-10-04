import { Dot } from 'lucide-react'

import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card'
import { Item, ItemContent, ItemGroup, ItemMedia } from '@/components/ui/item'
import { JsonBlock } from '@/components/ui/json-block'
import { Stack } from '@/components/ui/stack'
import { Heading } from '@/components/ui/typography'
import { AiRowsTable } from '../ai-rows-table'
import { DetailFields } from './detail-fields'
import { ImageGallery } from './image-gallery'
import { PrimitiveValue } from './primitive-value'
import { collectImages, isPrimitive, isRecord, labelOf } from './values'

const MAX_DEPTH = 3

function SectionBody({ value, depth }: { value: unknown; depth: number }) {
  if (Array.isArray(value)) {
    const images = collectImages(value)
    if (images.length === value.length && images.length > 0) {
      return <ImageGallery images={images} alt="" />
    }
    if (value.every(isRecord)) {
      return <AiRowsTable rows={value} />
    }
    if (value.every(isPrimitive)) {
      return (
        <ItemGroup className="gap-0">
          {value.map((entry, index) => (
            <Item key={index} size="sm" className="items-start px-0 py-1">
              <ItemMedia className="mt-0.5">
                <Dot />
              </ItemMedia>
              <ItemContent className="min-w-0">
                <PrimitiveValue value={entry} />
              </ItemContent>
            </Item>
          ))}
        </ItemGroup>
      )
    }
  }
  if (isRecord(value) && depth < MAX_DEPTH) {
    return <DetailFields record={value} depth={depth + 1} />
  }
  return <JsonBlock data={value} maxHeight="max-h-72" />
}

export function DetailSection({
  name,
  value,
  depth,
}: {
  name: string
  value: unknown
  depth: number
}) {
  if (depth > 0) {
    return (
      <Stack gap="sm">
        <Heading level={4} className="text-sm">
          {labelOf(name)}
        </Heading>
        <SectionBody value={value} depth={depth} />
      </Stack>
    )
  }
  return (
    <Card>
      <CardHeader>
        <CardTitle>{labelOf(name)}</CardTitle>
      </CardHeader>
      <CardContent>
        <SectionBody value={value} depth={depth} />
      </CardContent>
    </Card>
  )
}
