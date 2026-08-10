import { useQuery } from "@tanstack/react-query"
import { Activity, Globe, Shield, Cpu } from "lucide-react"
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from "@/components/ui/card"
import { Badge } from "@/components/ui/badge"
import { healthQueryOptions, profilesQueryOptions } from "@/lib/query"

export function DashboardPage() {
  const health = useQuery(healthQueryOptions)
  const profiles = useQuery(profilesQueryOptions)

  return (
    <div className="space-y-6">
      <div>
        <h2 className="text-2xl font-bold tracking-tight">Dashboard</h2>
        <p className="text-sm text-muted-foreground">
          Cosmium stealth browser engine overview
        </p>
      </div>

      <div className="grid gap-4 sm:grid-cols-2 lg:grid-cols-4">
        <Card>
          <CardHeader className="flex flex-row items-center justify-between pb-2">
            <CardTitle className="text-sm font-medium">Engine</CardTitle>
            <Activity className="size-4 text-muted-foreground" />
          </CardHeader>
          <CardContent>
            <Badge
              variant={
                health.data === true
                  ? "outline"
                  : health.data === false
                    ? "destructive"
                    : "secondary"
              }
            >
              {health.isLoading
                ? "checking…"
                : health.data
                  ? "connected"
                  : "offline"}
            </Badge>
            <p className="mt-1 text-xs text-muted-foreground">
              API server status
            </p>
          </CardContent>
        </Card>

        <Card>
          <CardHeader className="flex flex-row items-center justify-between pb-2">
            <CardTitle className="text-sm font-medium">Profiles</CardTitle>
            <Globe className="size-4 text-muted-foreground" />
          </CardHeader>
          <CardContent>
            <div className="text-2xl font-bold">
              {profiles.isLoading ? "…" : (profiles.data?.length ?? 0)}
            </div>
            <p className="text-xs text-muted-foreground">
              Browser fingerprints available
            </p>
          </CardContent>
        </Card>

        <Card>
          <CardHeader className="flex flex-row items-center justify-between pb-2">
            <CardTitle className="text-sm font-medium">Patches</CardTitle>
            <Shield className="size-4 text-muted-foreground" />
          </CardHeader>
          <CardContent>
            <div className="text-2xl font-bold">22</div>
            <p className="text-xs text-muted-foreground">
              C++ anti-detection patches
            </p>
          </CardContent>
        </Card>

        <Card>
          <CardHeader className="flex flex-row items-center justify-between pb-2">
            <CardTitle className="text-sm font-medium">Runtime</CardTitle>
            <Cpu className="size-4 text-muted-foreground" />
          </CardHeader>
          <CardContent>
            <div className="text-2xl font-bold">CDP</div>
            <p className="text-xs text-muted-foreground">
              Chrome DevTools Protocol
            </p>
          </CardContent>
        </Card>
      </div>

      {profiles.data && profiles.data.length > 0 && (
        <Card>
          <CardHeader>
            <CardTitle>Available Profiles</CardTitle>
            <CardDescription>
              Browser fingerprint profiles for stealth scraping
            </CardDescription>
          </CardHeader>
          <CardContent>
            <div className="flex flex-wrap gap-2">
              {profiles.data.map((p) => (
                <Badge key={p} variant="secondary" className="font-mono text-xs">
                  {p}
                </Badge>
              ))}
            </div>
          </CardContent>
        </Card>
      )}
    </div>
  )
}
