import { useEffect } from 'react'
import { Outlet, useNavigate, useRouterState } from '@tanstack/react-router'

import {
  Breadcrumb,
  BreadcrumbItem,
  BreadcrumbList,
  BreadcrumbPage,
} from '@/components/ui/breadcrumb'
import { TopBar } from '@/components/ui/page'
import { Separator } from '@/components/ui/separator'
import { SidebarInset, SidebarProvider, SidebarTrigger } from '@/components/ui/sidebar'
import { TooltipProvider } from '@/components/ui/tooltip'
import { useAuthStatus } from '@/stores/auth'
import { AppSidebar } from './app-sidebar'
import { titleForPath } from './nav-items'

export function AppShell() {
  const pathname = useRouterState({ select: (state) => state.location.pathname })
  const status = useAuthStatus()
  const navigate = useNavigate()

  useEffect(() => {
    if (status === 'signed-out') {
      void navigate({ to: '/login' })
    }
  }, [status, navigate])

  return (
    <TooltipProvider>
      <SidebarProvider>
        <AppSidebar />
        <SidebarInset>
          <TopBar>
            <SidebarTrigger />
            <Separator orientation="vertical" className="mr-2 h-4" />
            <Breadcrumb>
              <BreadcrumbList>
                <BreadcrumbItem>
                  <BreadcrumbPage>{titleForPath(pathname)}</BreadcrumbPage>
                </BreadcrumbItem>
              </BreadcrumbList>
            </Breadcrumb>
          </TopBar>
          <Outlet />
        </SidebarInset>
      </SidebarProvider>
    </TooltipProvider>
  )
}
