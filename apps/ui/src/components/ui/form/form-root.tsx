import * as React from 'react'

import { cn } from '@/libs/utils'
import { useFormContext } from './form-context'

export function FormRoot({ className, ...props }: Omit<React.ComponentProps<'form'>, 'onSubmit'>) {
  const form = useFormContext()
  return (
    <form
      data-slot="form-root"
      noValidate
      className={cn('flex flex-col gap-6', className)}
      onSubmit={(event) => {
        event.preventDefault()
        event.stopPropagation()
        void form.handleSubmit()
      }}
      {...props}
    />
  )
}
