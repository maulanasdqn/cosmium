import {
  Braces,
  FileText,
  Heading,
  Image,
  Link2,
  Sparkles,
  Table2,
  TextCursorInput,
} from 'lucide-react'
import type { LucideIcon } from 'lucide-react'
import type { ReactNode } from 'react'

import { Badge } from '@/components/ui/badge'
import { CodeBlock, TextBlock } from '@/components/ui/code-block'
import { JsonBlock } from '@/components/ui/json-block'
import { Empty, EmptyDescription, EmptyHeader, EmptyTitle } from '@/components/ui/empty'
import { Tabs, TabsContent, TabsList, TabsTrigger } from '@/components/ui/tabs'
import type { TAiState } from './ai-format'
import { AiResultView } from './ai-result-view'
import { HeadingsList } from './headings-list'
import { ImagesGrid } from './images-grid'
import { LinksTable } from './links-table'
import type { TPageData, TSelectorMatch } from './page-data'
import { TablesView } from './tables-view'

type TSection = {
  id: string
  label: string
  icon: LucideIcon
  count?: number
  content: ReactNode
}

function sectionsFor(data: TPageData, matches: TSelectorMatch[]): TSection[] {
  const sections: TSection[] = []
  if (data.text) {
    const content = <TextBlock value={data.text} maxHeight="max-h-[32rem]" />
    sections.push({ id: 'text', label: 'Text', icon: FileText, content })
  }
  if (data.links) {
    const content = <LinksTable links={data.links} />
    sections.push({ id: 'links', label: 'Links', icon: Link2, count: data.links.length, content })
  }
  if (data.headings) {
    const content = <HeadingsList headings={data.headings} />
    const count = data.headings.length
    sections.push({ id: 'headings', label: 'Headings', icon: Heading, count, content })
  }
  if (data.images) {
    const content = <ImagesGrid images={data.images} />
    sections.push({
      id: 'images',
      label: 'Images',
      icon: Image,
      count: data.images.length,
      content,
    })
  }
  if (data.tables) {
    const content = <TablesView tables={data.tables} />
    sections.push({
      id: 'tables',
      label: 'Tables',
      icon: Table2,
      count: data.tables.length,
      content,
    })
  }
  matches.forEach((match) => {
    const content = <CodeBlock value={match.values.join('\n') || 'No matches'} />
    const count = match.values.length
    sections.push({
      id: match.selector,
      label: match.selector,
      icon: TextCursorInput,
      count,
      content,
    })
  })
  return sections
}

export function ResultSections({
  data,
  matches,
  extracted,
  ai,
}: {
  data: TPageData
  matches: TSelectorMatch[]
  extracted: Record<string, unknown>
  ai: TAiState
}) {
  const rawSection: TSection = {
    id: 'raw',
    label: 'Raw',
    icon: Braces,
    content: <JsonBlock data={extracted} maxHeight="max-h-[32rem]" />,
  }
  const raw = [...sectionsFor(data, matches), rawSection]
  const aiSection: TSection = {
    id: 'ai',
    label: 'AI result',
    icon: Sparkles,
    content: <AiResultView ai={ai} />,
  }
  const sections = ai.status === 'off' ? raw : [aiSection, ...raw]
  const [first] = sections
  if (!first) {
    return (
      <Empty className="border">
        <EmptyHeader>
          <EmptyTitle>Nothing collected</EmptyTitle>
          <EmptyDescription>
            The page loaded, but none of the chosen data was found.
          </EmptyDescription>
        </EmptyHeader>
      </Empty>
    )
  }
  return (
    <Tabs key={first.id} defaultValue={first.id}>
      <TabsList className="flex-wrap">
        {sections.map((section) => (
          <TabsTrigger key={section.id} value={section.id}>
            <section.icon />
            {section.label}
            {section.count === undefined ? null : (
              <Badge variant="secondary">{section.count}</Badge>
            )}
          </TabsTrigger>
        ))}
      </TabsList>
      {sections.map((section) => (
        <TabsContent key={section.id} value={section.id} className="pt-2">
          {section.content}
        </TabsContent>
      ))}
    </Tabs>
  )
}
