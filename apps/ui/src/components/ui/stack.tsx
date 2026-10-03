import * as React from 'react'
import { cva, type VariantProps } from 'class-variance-authority'

import { cn } from '@/libs/utils'

const stackVariants = cva('flex', {
  variants: {
    direction: {
      row: 'flex-row',
      column: 'flex-col',
      responsive: 'flex-col md:flex-row',
    },
    gap: {
      none: 'gap-0',
      xs: 'gap-1',
      sm: 'gap-2',
      md: 'gap-4',
      lg: 'gap-6',
      xl: 'gap-8',
    },
    align: {
      start: 'items-start',
      center: 'items-center',
      end: 'items-end',
      stretch: 'items-stretch',
      baseline: 'items-baseline',
    },
    justify: {
      start: 'justify-start',
      center: 'justify-center',
      end: 'justify-end',
      between: 'justify-between',
    },
    wrap: {
      true: 'flex-wrap',
      false: 'flex-nowrap',
    },
    grow: {
      true: 'flex-1 min-w-0',
      false: '',
    },
  },
  defaultVariants: {
    direction: 'column',
    gap: 'md',
    align: 'stretch',
    justify: 'start',
    wrap: false,
    grow: false,
  },
})

function Stack({
  className,
  direction,
  gap,
  align,
  justify,
  wrap,
  grow,
  ...props
}: React.ComponentProps<'div'> & VariantProps<typeof stackVariants>) {
  return (
    <div
      data-slot="stack"
      className={cn(stackVariants({ direction, gap, align, justify, wrap, grow }), className)}
      {...props}
    />
  )
}

const gridVariants = cva('grid', {
  variants: {
    columns: {
      1: 'grid-cols-1',
      2: 'grid-cols-1 md:grid-cols-2',
      3: 'grid-cols-1 md:grid-cols-2 xl:grid-cols-3',
      4: 'grid-cols-1 sm:grid-cols-2 xl:grid-cols-4',
    },
    gap: {
      sm: 'gap-2',
      md: 'gap-4',
      lg: 'gap-6',
    },
  },
  defaultVariants: {
    columns: 2,
    gap: 'md',
  },
})

function Grid({
  className,
  columns,
  gap,
  ...props
}: React.ComponentProps<'div'> & VariantProps<typeof gridVariants>) {
  return (
    <div data-slot="grid" className={cn(gridVariants({ columns, gap }), className)} {...props} />
  )
}

export { Grid, Stack, gridVariants, stackVariants }
