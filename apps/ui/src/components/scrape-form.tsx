import { useState } from "react"
import { useMutation, useQuery } from "@tanstack/react-query"
import { Loader2 } from "lucide-react"
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
import { Switch } from "@/components/ui/switch"
import { Textarea } from "@/components/ui/textarea"
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select"
import { scrape, type ScrapeResponse } from "@/lib/api"
import { profilesQueryOptions } from "@/lib/query"

type Props = {
  onResult: (result: ScrapeResponse) => void
}

export function ScrapeForm({ onResult }: Props) {
  const { data: profiles = [], isLoading: profilesLoading } =
    useQuery(profilesQueryOptions)

  const [profile, setProfile] = useState("")
  const [url, setUrl] = useState("")
  const [headful, setHeadful] = useState(false)
  const [screenshot, setScreenshot] = useState(true)
  const [waitMs, setWaitMs] = useState("3000")
  const [extract, setExtract] = useState("")
  const [script, setScript] = useState("")
  const [proxy, setProxy] = useState("")

  const scrapeMutation = useMutation({
    mutationFn: scrape,
    onSuccess: onResult,
  })

  const isValidUrl = (() => {
    try {
      const u = new URL(url.trim())
      return u.protocol === "http:" || u.protocol === "https:"
    } catch {
      return false
    }
  })()

  function handleSubmit(e: React.FormEvent) {
    e.preventDefault()
    if (scrapeMutation.isPending || !isValidUrl || !profile) return

    scrapeMutation.mutate({
      url: url.trim(),
      profile,
      screenshot,
      headful,
      wait_ms: parseInt(waitMs) || 0,
      extract: extract
        .split(",")
        .map((s) => s.trim())
        .filter(Boolean),
      script: script.trim() || undefined,
      proxy: proxy.trim() || undefined,
      include_html: true,
    })
  }

  return (
    <Card>
      <CardHeader>
        <CardTitle>Scrape a page</CardTitle>
        <CardDescription>
          Enter a URL, pick a profile, and test your stealth scraping setup.
        </CardDescription>
      </CardHeader>
      <CardContent>
        <form onSubmit={handleSubmit} className="space-y-5">
          <div className="grid gap-4 sm:grid-cols-2">
            <div className="space-y-2">
              <Label htmlFor="profile">Profile</Label>
              <Select
                value={profile}
                onValueChange={(v) => setProfile(v ?? "")}
                disabled={profilesLoading}
              >
                <SelectTrigger id="profile" className="w-full">
                  <SelectValue
                    placeholder={
                      profilesLoading ? "Loading…" : "Select a profile"
                    }
                  />
                </SelectTrigger>
                <SelectContent>
                  {profiles.map((p) => (
                    <SelectItem key={p} value={p}>
                      {p}
                    </SelectItem>
                  ))}
                </SelectContent>
              </Select>
            </div>
            <div className="space-y-2">
              <Label htmlFor="url">URL</Label>
              <Input
                id="url"
                type="url"
                value={url}
                onChange={(e) => setUrl(e.target.value)}
                placeholder="https://example.com/page"
                className="font-mono text-sm"
              />
            </div>
          </div>

          <div className="grid gap-4 sm:grid-cols-3">
            <div className="flex items-center justify-between rounded-lg border p-3">
              <div>
                <Label htmlFor="headful" className="text-sm">
                  Headful
                </Label>
                <p className="text-xs text-muted-foreground">
                  Show browser window
                </p>
              </div>
              <Switch
                id="headful"
                checked={headful}
                onCheckedChange={setHeadful}
              />
            </div>
            <div className="flex items-center justify-between rounded-lg border p-3">
              <div>
                <Label htmlFor="screenshot" className="text-sm">
                  Screenshot
                </Label>
                <p className="text-xs text-muted-foreground">
                  Capture JPEG
                </p>
              </div>
              <Switch
                id="screenshot"
                checked={screenshot}
                onCheckedChange={setScreenshot}
              />
            </div>
            <div className="space-y-2">
              <Label htmlFor="wait_ms">Wait (ms)</Label>
              <Input
                id="wait_ms"
                type="number"
                value={waitMs}
                onChange={(e) => setWaitMs(e.target.value)}
                min={0}
                step={500}
              />
            </div>
          </div>

          <div className="space-y-2">
            <Label htmlFor="extract">
              Extract selectors{" "}
              <span className="font-normal text-muted-foreground">
                (comma-separated)
              </span>
            </Label>
            <Input
              id="extract"
              value={extract}
              onChange={(e) => setExtract(e.target.value)}
              placeholder="h1, .price, .title"
            />
          </div>

          <div className="grid gap-4 sm:grid-cols-2">
            <div className="space-y-2">
              <Label htmlFor="script">JavaScript</Label>
              <Textarea
                id="script"
                value={script}
                onChange={(e) => setScript(e.target.value)}
                placeholder="document.title"
                rows={2}
                className="font-mono text-sm"
              />
            </div>
            <div className="space-y-2">
              <Label htmlFor="proxy">Proxy</Label>
              <Input
                id="proxy"
                value={proxy}
                onChange={(e) => setProxy(e.target.value)}
                placeholder="http://user:pass@host:port"
                className="font-mono text-sm"
              />
            </div>
          </div>

          <div className="flex items-center gap-3">
            <Button
              type="submit"
              disabled={scrapeMutation.isPending || !isValidUrl || !profile}
            >
              {scrapeMutation.isPending && (
                <Loader2 className="size-4 animate-spin" />
              )}
              {scrapeMutation.isPending ? "Scraping…" : "Scrape"}
            </Button>
            {url.trim() && !isValidUrl && (
              <p className="text-sm text-destructive">Not a valid URL</p>
            )}
          </div>

          {scrapeMutation.isError && (
            <div className="rounded-lg border border-destructive bg-destructive/10 p-3 text-sm text-destructive">
              {scrapeMutation.error.message}
            </div>
          )}
        </form>
      </CardContent>
    </Card>
  )
}
