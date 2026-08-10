import { useLocation } from "@tanstack/react-router"
import {
  Globe,
  Search,
  LayoutDashboard,
  LogOut,
  Sparkles,
} from "lucide-react"
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
} from "@/components/ui/sidebar"
import { Button } from "@/components/ui/button"
import { signOut } from "@/lib/auth"
import { useRouter } from "@tanstack/react-router"

const NAV = [
  { to: "/" as const, label: "Dashboard", icon: LayoutDashboard },
  { to: "/scrape" as const, label: "Single Scrape", icon: Search },
  { to: "/generate" as const, label: "Generate Profile", icon: Sparkles },
]

export function AppSidebar() {
  const location = useLocation()
  const router = useRouter()

  function handleSignOut() {
    signOut()
    router.navigate({ to: "/login" })
  }

  return (
    <Sidebar>
      <SidebarHeader>
        <div className="flex items-center gap-2 px-2 py-3">
          <div className="flex size-8 items-center justify-center rounded-lg bg-primary text-primary-foreground">
            <Globe className="size-4" />
          </div>
          <div className="leading-tight">
            <p className="text-sm font-semibold">Cosmium</p>
            <p className="text-[11px] text-muted-foreground">Engine</p>
          </div>
        </div>
      </SidebarHeader>

      <SidebarContent>
        <SidebarGroup>
          <SidebarGroupLabel>Navigation</SidebarGroupLabel>
          <SidebarGroupContent>
            <SidebarMenu>
              {NAV.map((item) => (
                <SidebarMenuItem key={item.to}>
                  <SidebarMenuButton
                    isActive={location.pathname === item.to}
                    onClick={() =>
                      router.navigate({ to: item.to })
                    }
                  >
                    <item.icon className="size-4" />
                    <span>{item.label}</span>
                  </SidebarMenuButton>
                </SidebarMenuItem>
              ))}
            </SidebarMenu>
          </SidebarGroupContent>
        </SidebarGroup>
      </SidebarContent>

      <SidebarFooter>
        <Button
          variant="ghost"
          size="sm"
          className="w-full justify-start gap-2 text-muted-foreground"
          onClick={handleSignOut}
        >
          <LogOut className="size-4" />
          Sign out
        </Button>
      </SidebarFooter>
    </Sidebar>
  )
}
