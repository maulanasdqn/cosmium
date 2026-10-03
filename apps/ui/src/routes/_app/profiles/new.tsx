import { createFileRoute } from '@tanstack/react-router'

import { GenerateProfilePage } from '@/components/features/profiles'

export const Route = createFileRoute('/_app/profiles/new')({
  component: GenerateProfilePage,
})
