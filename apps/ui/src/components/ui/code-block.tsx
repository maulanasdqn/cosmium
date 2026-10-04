import * as React from 'react'

import { cn } from '@/libs/utils'

function CodeBlock({
  className,
  value,
  maxHeight = 'max-h-96',
  ...props
}: Omit<React.ComponentProps<'pre'>, 'children'> & { value: string; maxHeight?: string }) {
  return (
    <div
      data-slot="code-block"
      className={cn('overflow-auto overscroll-contain rounded-lg border bg-muted/40', maxHeight)}
    >
      <pre
        className={cn('p-4 font-mono text-xs leading-5 whitespace-pre-wrap break-words', className)}
        {...props}
      >
        {value}
      </pre>
    </div>
  )
}

function TextBlock({
  className,
  value,
  maxHeight = 'max-h-96',
  ...props
}: Omit<React.ComponentProps<'div'>, 'children'> & { value: string; maxHeight?: string }) {
  return (
    <div
      data-slot="text-block"
      className={cn(
        'overflow-auto overscroll-contain rounded-lg border bg-muted/20 p-4 text-sm leading-6 whitespace-pre-line break-words',
        maxHeight,
        className,
      )}
      {...props}
    >
      {value}
    </div>
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

export { CodeBlock, ImagePreview, TextBlock }
