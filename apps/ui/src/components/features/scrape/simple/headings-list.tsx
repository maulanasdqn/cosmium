import { Badge } from '@/components/ui/badge'
import { Item, ItemContent, ItemGroup, ItemMedia, ItemTitle } from '@/components/ui/item'
import { cn } from '@/libs/utils'
import type { TPageData } from './page-data'

const indent: Record<number, string> = { 1: 'ml-0', 2: 'ml-6', 3: 'ml-12' }

export function HeadingsList({ headings }: { headings: NonNullable<TPageData['headings']> }) {
  return (
    <ItemGroup>
      {headings.map((heading, index) => (
        <Item
          key={`${index}-${heading.text}`}
          size="sm"
          className={cn('py-1.5', indent[heading.level])}
        >
          <ItemMedia>
            <Badge variant="outline">H{heading.level}</Badge>
          </ItemMedia>
          <ItemContent>
            <ItemTitle className={heading.level === 1 ? 'text-base' : undefined}>
              {heading.text}
            </ItemTitle>
          </ItemContent>
        </Item>
      ))}
    </ItemGroup>
  )
}
