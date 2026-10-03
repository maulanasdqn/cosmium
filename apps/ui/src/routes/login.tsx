import { createFileRoute, redirect } from '@tanstack/react-router'

import { LoginPage } from '@/components/features/auth'
import { authStore } from '@/stores/auth'

export const Route = createFileRoute('/login')({
  beforeLoad: () => {
    if (authStore.state.status === 'signed-in') {
      throw redirect({ to: '/' })
    }
  },
  component: LoginPage,
})
