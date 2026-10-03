import { createFileRoute } from '@tanstack/react-router'
import { z } from 'zod'

import { TestsPage } from '@/components/features/tests'

const testsSearchSchema = z.object({
  profile: z.string().optional(),
})

export const Route = createFileRoute('/_app/tests')({
  validateSearch: testsSearchSchema,
  component: TestsRoute,
})

function TestsRoute() {
  const { profile } = Route.useSearch()
  return <TestsPage profile={profile} />
}
