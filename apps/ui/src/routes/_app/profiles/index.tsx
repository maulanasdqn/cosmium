import { createFileRoute } from '@tanstack/react-router'

import { ProfilesPage } from '@/components/features/profiles'

export const Route = createFileRoute('/_app/profiles/')({
  component: ProfilesPage,
})
