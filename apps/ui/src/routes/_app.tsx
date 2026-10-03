import { createFileRoute, redirect } from '@tanstack/react-router'

import { authService } from '@/apis/auth'
import { AppShell } from '@/components/layout'
import { authStore, markSignedIn, signOut } from '@/stores/auth'

async function isKeyValid(apiKey: string): Promise<boolean> {
  try {
    await authService.verify({ apiKey })
    return true
  } catch {
    return false
  }
}

export const Route = createFileRoute('/_app')({
  beforeLoad: async () => {
    const { apiKey, status } = authStore.state
    if (!apiKey) {
      throw redirect({ to: '/login' })
    }
    if (status === 'checking') {
      if (await isKeyValid(apiKey)) {
        markSignedIn()
      } else {
        signOut()
        throw redirect({ to: '/login' })
      }
    }
  },
  component: AppShell,
})
