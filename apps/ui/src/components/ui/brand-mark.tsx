import * as React from 'react'

import { cn } from '@/libs/utils'

function BrandMark({
  className,
  icon,
  title,
  subtitle,
  ...props
}: Omit<React.ComponentProps<'div'>, 'title'> & {
  icon: React.ReactNode
  title: React.ReactNode
  subtitle?: React.ReactNode
}) {
  return (
    <div
      data-slot="brand-mark"
      className={cn('flex items-center gap-2 px-2 py-2', className)}
      {...props}
    >
      <div className="flex size-8 shrink-0 items-center justify-center rounded-lg bg-primary text-primary-foreground [&_svg]:size-4">
        {icon}
      </div>
      <div className="grid min-w-0 leading-tight">
        <span className="truncate text-sm font-semibold">{title}</span>
        {subtitle ? (
          <span className="truncate text-xs text-muted-foreground">{subtitle}</span>
        ) : null}
      </div>
    </div>
  )
}

export { BrandMark }
