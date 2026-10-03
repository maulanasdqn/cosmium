import { Link, useNavigate, useRouterState } from '@tanstack/react-router'
import { Globe, LogOut } from 'lucide-react'

import { BrandMark } from '@/components/ui/brand-mark'
import {
  Sidebar,
  SidebarContent,
  SidebarFooter,
  SidebarGroup,
  SidebarGroupContent,
  SidebarGroupLabel,
  SidebarHeader,
  SidebarMenu,
  SidebarMenuButton,
  SidebarMenuItem,
} from '@/components/ui/sidebar'
import { signOut } from '@/stores/auth'
import { navItems } from './nav-items'

export function AppSidebar() {
  const pathname = useRouterState({ select: (state) => state.location.pathname })
  const navigate = useNavigate()

  return (
    <Sidebar>
      <SidebarHeader>
        <BrandMark icon={<Globe />} title="Cosmium" subtitle="Stealth browser platform" />
      </SidebarHeader>
      <SidebarContent>
        <SidebarGroup>
          <SidebarGroupLabel>Platform</SidebarGroupLabel>
          <SidebarGroupContent>
            <SidebarMenu>
              {navItems.map((item) => (
                <SidebarMenuItem key={item.to}>
                  <SidebarMenuButton
                    isActive={item.to === '/' ? pathname === '/' : pathname.startsWith(item.to)}
                    render={<Link to={item.to} />}
                  >
                    <item.icon />
                    {item.label}
                  </SidebarMenuButton>
                </SidebarMenuItem>
              ))}
            </SidebarMenu>
          </SidebarGroupContent>
        </SidebarGroup>
      </SidebarContent>
      <SidebarFooter>
        <SidebarMenu>
          <SidebarMenuItem>
            <SidebarMenuButton
              onClick={() => {
                signOut()
                void navigate({ to: '/login' })
              }}
            >
              <LogOut />
              Sign out
            </SidebarMenuButton>
          </SidebarMenuItem>
        </SidebarMenu>
      </SidebarFooter>
    </Sidebar>
  )
}
