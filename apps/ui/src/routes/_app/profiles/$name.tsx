import { createFileRoute } from '@tanstack/react-router'

import { ProfileDetailPage } from '@/components/features/profiles'

export const Route = createFileRoute('/_app/profiles/$name')({
  component: ProfileDetailRoute,
})

function ProfileDetailRoute() {
  const { name } = Route.useParams()
  return <ProfileDetailPage name={name} />
}
