import { Outlet, createRootRouteWithContext } from '@tanstack/react-router'
import type { QueryClient } from '@tanstack/react-query'

import { Toaster } from '@/components/ui/sonner'

export type TRouterContext = {
  queryClient: QueryClient
}

export const Route = createRootRouteWithContext<TRouterContext>()({
  component: RootLayout,
})

function RootLayout() {
  return (
    <>
      <Outlet />
      <Toaster richColors position="bottom-right" />
    </>
  )
}
