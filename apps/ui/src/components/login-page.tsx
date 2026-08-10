import { useState } from "react"
import { useRouter } from "@tanstack/react-router"
import { Globe, Loader2, KeyRound } from "lucide-react"
import { Button } from "@/components/ui/button"
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from "@/components/ui/card"
import { Input } from "@/components/ui/input"
import { Label } from "@/components/ui/label"
import { verifyToken } from "@/lib/api"
import { signIn } from "@/lib/auth"

export function LoginPage() {
  const router = useRouter()
  const [token, setToken] = useState("")
  const [busy, setBusy] = useState(false)
  const [error, setError] = useState("")

  async function handleSubmit(e: React.FormEvent) {
    e.preventDefault()
    const trimmed = token.trim()
    if (!trimmed || busy) return

    setBusy(true)
    setError("")

    const ok = await verifyToken(trimmed)
    if (ok) {
      signIn(trimmed)
      router.navigate({ to: "/" })
    } else {
      setError("That token was rejected. Check and try again.")
    }
    setBusy(false)
  }

  return (
    <div className="flex min-h-screen items-center justify-center bg-background p-4">
      <div className="w-full max-w-sm space-y-6">
        <div className="flex flex-col items-center gap-2">
          <div className="flex size-12 items-center justify-center rounded-xl bg-primary text-primary-foreground">
            <Globe className="size-6" />
          </div>
          <h1 className="text-2xl font-bold tracking-tight">Cosmium</h1>
          <p className="text-sm text-muted-foreground">
            Stealth browser scraping engine
          </p>
        </div>

        <Card>
          <CardHeader className="text-center">
            <CardTitle>Sign in</CardTitle>
            <CardDescription>
              Paste your API token to access the scraping dashboard
            </CardDescription>
          </CardHeader>
          <CardContent>
            <form onSubmit={handleSubmit} className="space-y-4">
              <div className="space-y-2">
                <Label htmlFor="token">API token</Label>
                <div className="relative">
                  <KeyRound className="absolute left-3 top-1/2 size-4 -translate-y-1/2 text-muted-foreground" />
                  <Input
                    id="token"
                    type="password"
                    value={token}
                    onChange={(e) => setToken(e.target.value)}
                    placeholder="dev-key"
                    className="pl-10"
                    autoFocus
                  />
                </div>
              </div>

              {error && (
                <p className="text-sm text-destructive">{error}</p>
              )}

              <Button
                type="submit"
                className="w-full"
                disabled={busy || !token.trim()}
              >
                {busy && <Loader2 className="size-4 animate-spin" />}
                {busy ? "Checking…" : "Sign in"}
              </Button>
            </form>
          </CardContent>
        </Card>

        <p className="text-center text-xs text-muted-foreground">
          The token is kept in this browser only and sent as{" "}
          <code className="rounded bg-muted px-1 py-0.5 font-mono text-[11px]">
            x-api-key
          </code>{" "}
          on every request.
        </p>
      </div>
    </div>
  )
}
