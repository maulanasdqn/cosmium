import {
  createRouter,
  createRoute,
  createRootRoute,
  redirect,
  Outlet,
  useLocation,
} from "@tanstack/react-router"
import { Toaster } from "@/components/ui/sonner"
import { SidebarProvider, SidebarTrigger } from "@/components/ui/sidebar"
import { TooltipProvider } from "@/components/ui/tooltip"
import { Separator } from "@/components/ui/separator"
import { AppSidebar } from "@/components/app-sidebar"
import { DashboardPage } from "@/components/dashboard-page"
import { ScrapePage } from "@/components/scrape-page"
import { GenerateProfilePage } from "@/components/generate-profile-page"
import { LoginPage } from "@/components/login-page"
import { authStore, setAuthStatus, signOut } from "@/lib/auth"
import { verifyToken } from "@/lib/api"

const rootRoute = createRootRoute({
  component: () => (
    <>
      <Outlet />
      <Toaster />
    </>
  ),
})

const loginRoute = createRoute({
  getParentRoute: () => rootRoute,
  path: "/login",
  beforeLoad: () => {
    if (authStore.state.status === "in") {
      throw redirect({ to: "/" })
    }
  },
  component: LoginPage,
})

const PAGE_TITLES: Record<string, string> = {
  "/": "Dashboard",
  "/scrape": "Single Scrape",
  "/generate": "Generate Profile",
}

function AuthLayout() {
  const location = useLocation()

  return (
    <TooltipProvider>
      <SidebarProvider>
        <div className="flex min-h-screen w-full bg-background text-foreground">
          <AppSidebar />
          <div className="flex flex-1 flex-col">
            <header className="flex h-12 items-center gap-2 border-b px-4">
              <SidebarTrigger />
              <Separator orientation="vertical" className="h-4" />
              <span className="text-sm font-medium">
                {PAGE_TITLES[location.pathname] ?? "Cosmium"}
              </span>
            </header>
            <main className="flex-1 overflow-auto p-6">
              <Outlet />
            </main>
          </div>
        </div>
      </SidebarProvider>
    </TooltipProvider>
  )
}

const authLayout = createRoute({
  getParentRoute: () => rootRoute,
  id: "auth",
  beforeLoad: async () => {
    const { status, apiKey } = authStore.state

    if (!apiKey) {
      setAuthStatus("out")
      throw redirect({ to: "/login" })
    }

    if (status === "loading") {
      const ok = await verifyToken(apiKey)
      if (ok) {
        setAuthStatus("in")
      } else {
        signOut()
        throw redirect({ to: "/login" })
      }
    }

    if (status === "out") {
      throw redirect({ to: "/login" })
    }
  },
  component: AuthLayout,
})

const dashboardRoute = createRoute({
  getParentRoute: () => authLayout,
  path: "/",
  component: DashboardPage,
})

const scrapeRoute = createRoute({
  getParentRoute: () => authLayout,
  path: "/scrape",
  component: ScrapePage,
})

const generateRoute = createRoute({
  getParentRoute: () => authLayout,
  path: "/generate",
  component: GenerateProfilePage,
})

const routeTree = rootRoute.addChildren([
  loginRoute,
  authLayout.addChildren([dashboardRoute, scrapeRoute, generateRoute]),
])

export const router = createRouter({ routeTree })

declare module "@tanstack/react-router" {
  interface Register {
    router: typeof router
  }
}
