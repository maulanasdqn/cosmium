import * as React from 'react'

import { cn } from '@/libs/utils'

function Page({ className, ...props }: React.ComponentProps<'div'>) {
  return (
    <div
      data-slot="page"
      className={cn('mx-auto flex w-full max-w-7xl flex-col gap-6 p-4 md:p-6', className)}
      {...props}
    />
  )
}

function PageHeader({
  className,
  title,
  description,
  actions,
  ...props
}: Omit<React.ComponentProps<'div'>, 'title'> & {
  title: React.ReactNode
  description?: React.ReactNode
  actions?: React.ReactNode
}) {
  return (
    <div
      data-slot="page-header"
      className={cn('flex flex-col gap-3 md:flex-row md:items-end md:justify-between', className)}
      {...props}
    >
      <div className="flex min-w-0 flex-col gap-1">
        <h1 className="text-2xl font-semibold tracking-tight">{title}</h1>
        {description ? <p className="text-sm text-muted-foreground">{description}</p> : null}
      </div>
      {actions ? <div className="flex flex-wrap items-center gap-2">{actions}</div> : null}
    </div>
  )
}

function TopBar({ className, ...props }: React.ComponentProps<'header'>) {
  return (
    <header
      data-slot="top-bar"
      className={cn(
        'sticky top-0 z-10 flex h-14 shrink-0 items-center gap-2 border-b bg-background/80 px-4 backdrop-blur',
        className,
      )}
      {...props}
    />
  )
}

function CenteredScreen({ className, ...props }: React.ComponentProps<'div'>) {
  return (
    <div
      data-slot="centered-screen"
      className={cn('flex min-h-svh items-center justify-center bg-muted/30 p-4', className)}
      {...props}
    />
  )
}

export { CenteredScreen, Page, PageHeader, TopBar }
