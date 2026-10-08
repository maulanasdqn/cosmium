import type { ReactNode } from 'react'

import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card'
import { Grid } from '@/components/ui/stack'

export function EditSectionCard({
  title,
  description,
  children,
}: {
  title: string
  description: string
  children: ReactNode
}) {
  return (
    <Card>
      <CardHeader>
        <CardTitle>{title}</CardTitle>
        <CardDescription>{description}</CardDescription>
      </CardHeader>
      <CardContent>
        <Grid columns={2}>{children}</Grid>
      </CardContent>
    </Card>
  )
}
