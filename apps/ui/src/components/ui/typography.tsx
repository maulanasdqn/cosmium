import * as React from 'react'
import { cva, type VariantProps } from 'class-variance-authority'

import { cn } from '@/libs/utils'

const headingVariants = cva('font-semibold tracking-tight text-foreground', {
  variants: {
    level: {
      1: 'text-3xl',
      2: 'text-2xl',
      3: 'text-lg',
      4: 'text-base',
    },
  },
  defaultVariants: {
    level: 2,
  },
})

type THeadingLevel = 1 | 2 | 3 | 4

function Heading({
  className,
  level,
  ...props
}: React.ComponentProps<'h2'> & VariantProps<typeof headingVariants>) {
  const Tag = `h${(level ?? 2) as THeadingLevel}` as const
  return (
    <Tag data-slot="heading" className={cn(headingVariants({ level }), className)} {...props} />
  )
}

const textVariants = cva('text-foreground', {
  variants: {
    variant: {
      default: 'text-sm leading-6',
      lead: 'text-base text-muted-foreground',
      muted: 'text-sm text-muted-foreground',
      small: 'text-xs text-muted-foreground',
      mono: 'font-mono text-xs break-all',
      error: 'text-sm text-destructive',
    },
    weight: {
      normal: 'font-normal',
      medium: 'font-medium',
      semibold: 'font-semibold',
    },
    truncate: {
      true: 'truncate',
      false: '',
    },
  },
  defaultVariants: {
    variant: 'default',
    weight: 'normal',
    truncate: false,
  },
})

function Text({
  className,
  variant,
  weight,
  truncate,
  inline = false,
  ...props
}: React.ComponentProps<'p'> & VariantProps<typeof textVariants> & { inline?: boolean }) {
  const Tag = inline ? 'span' : 'p'
  return (
    <Tag
      data-slot="text"
      className={cn(textVariants({ variant, weight, truncate }), className)}
      {...props}
    />
  )
}

function InlineCode({ className, ...props }: React.ComponentProps<'code'>) {
  return (
    <code
      data-slot="inline-code"
      className={cn('rounded bg-muted px-1.5 py-0.5 font-mono text-xs', className)}
      {...props}
    />
  )
}

export { Heading, InlineCode, Text, headingVariants, textVariants }
