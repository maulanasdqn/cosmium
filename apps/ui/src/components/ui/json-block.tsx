import * as React from 'react'

import { cn } from '@/libs/utils'

type TTokenKind = 'key' | 'string' | 'number' | 'literal' | 'plain'

type TToken = {
  kind: TTokenKind
  text: string
}

const TOKEN =
  /("(?:\\u[0-9a-fA-F]{4}|\\[^u]|[^\\"])*")(\s*:)?|\b(true|false|null)\b|(-?\d+(?:\.\d+)?(?:[eE][+-]?\d+)?)/g

const tokenClass: Record<TTokenKind, string> = {
  key: 'text-sky-700 dark:text-sky-300',
  string: 'text-emerald-700 dark:text-emerald-300',
  number: 'text-amber-700 dark:text-amber-300',
  literal: 'text-violet-700 dark:text-violet-300',
  plain: 'text-muted-foreground',
}

function tokenize(source: string): TToken[] {
  const tokens: TToken[] = []
  let cursor = 0
  for (const match of source.matchAll(TOKEN)) {
    const start = match.index
    if (start > cursor) {
      tokens.push({ kind: 'plain', text: source.slice(cursor, start) })
    }
    const [whole, quoted, colon, literal] = match
    if (quoted !== undefined) {
      tokens.push({ kind: colon ? 'key' : 'string', text: quoted })
      if (colon) {
        tokens.push({ kind: 'plain', text: colon })
      }
    } else {
      tokens.push({ kind: literal !== undefined ? 'literal' : 'number', text: whole })
    }
    cursor = start + whole.length
  }
  if (cursor < source.length) {
    tokens.push({ kind: 'plain', text: source.slice(cursor) })
  }
  return tokens
}

function JsonBlock({
  className,
  data,
  maxHeight = 'max-h-96',
  ...props
}: Omit<React.ComponentProps<'pre'>, 'children'> & { data: unknown; maxHeight?: string }) {
  const tokens = React.useMemo(() => tokenize(JSON.stringify(data, null, 2) ?? 'null'), [data])
  return (
    <div
      data-slot="json-block"
      className={cn('overflow-auto overscroll-contain rounded-lg border bg-muted/40', maxHeight)}
    >
      <pre
        className={cn('p-4 font-mono text-xs leading-5 whitespace-pre-wrap break-words', className)}
        {...props}
      >
        {tokens.map((token, index) => (
          <span key={index} className={tokenClass[token.kind]}>
            {token.text}
          </span>
        ))}
      </pre>
    </div>
  )
}

export { JsonBlock }
