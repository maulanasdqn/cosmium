import * as React from 'react'

import { cn } from '@/libs/utils'
import { ScrollArea } from '@/components/ui/scroll-area'

function CodeBlock({
  className,
  value,
  maxHeight = 'max-h-96',
  ...props
}: Omit<React.ComponentProps<'pre'>, 'children'> & { value: string; maxHeight?: string }) {
  return (
    <ScrollArea className={cn('rounded-lg border bg-muted/40', maxHeight)}>
      <pre
        data-slot="code-block"
        className={cn('p-4 font-mono text-xs leading-5 whitespace-pre-wrap break-all', className)}
        {...props}
      >
        {value}
      </pre>
    </ScrollArea>
  )
}

function ImagePreview({ className, alt, ...props }: React.ComponentProps<'img'>) {
  return (
    <img
      data-slot="image-preview"
      alt={alt}
      className={cn('w-full rounded-lg border object-contain', className)}
      {...props}
    />
  )
}

export { CodeBlock, ImagePreview }
