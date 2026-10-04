import * as React from 'react'

import { cn } from '@/libs/utils'

function DescriptionList({ className, ...props }: React.ComponentProps<'dl'>) {
  return (
    <dl
      data-slot="description-list"
      className={cn('grid grid-cols-1 divide-y divide-border/60', className)}
      {...props}
    />
  )
}

function DescriptionItem({
  className,
  label,
  children,
  ...props
}: Omit<React.ComponentProps<'div'>, 'children'> & {
  label: React.ReactNode
  children: React.ReactNode
}) {
  return (
    <div
      data-slot="description-item"
      className={cn(
        'grid grid-cols-1 gap-1 py-2.5 first:pt-0 last:pb-0 sm:grid-cols-[minmax(8rem,14rem)_minmax(0,1fr)] sm:gap-4',
        className,
      )}
      {...props}
    >
      <dt className="text-sm text-muted-foreground">{label}</dt>
      <dd className="min-w-0 text-sm break-words">{children}</dd>
    </div>
  )
}

export { DescriptionItem, DescriptionList }
