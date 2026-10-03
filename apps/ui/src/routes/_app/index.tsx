import { createFileRoute } from '@tanstack/react-router'

import { DashboardPage } from '@/components/features/dashboard'

export const Route = createFileRoute('/_app/')({
  component: DashboardPage,
})
