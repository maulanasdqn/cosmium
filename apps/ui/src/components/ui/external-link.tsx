import * as React from 'react'

import { cn } from '@/libs/utils'

function ExternalLink({ className, ...props }: Omit<React.ComponentProps<'a'>, 'target' | 'rel'>) {
  return (
    <a
      data-slot="external-link"
      target="_blank"
      rel="noreferrer noopener"
      className={cn('underline-offset-4 hover:underline', className)}
      {...props}
    />
  )
}

export { ExternalLink }
