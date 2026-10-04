import { createFileRoute } from '@tanstack/react-router'

import { ResultsPage } from '@/components/features/results'

export const Route = createFileRoute('/_app/results')({
  component: ResultsPage,
})
