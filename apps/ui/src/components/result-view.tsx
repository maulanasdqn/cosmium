import { useState } from "react"
import type { ScrapeResponse } from "@/lib/api"
import { Badge } from "@/components/ui/badge"
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from "@/components/ui/card"
import { Button } from "@/components/ui/button"
import { ChevronDown, ChevronRight } from "lucide-react"

export function ResultView({ data }: { data: ScrapeResponse }) {
  const [showHtml, setShowHtml] = useState(false)
  const hasExtracted = Object.keys(data.extracted).length > 0

  return (
    <Card>
      <CardHeader>
        <CardTitle>Result</CardTitle>
        <CardDescription className="font-mono text-xs break-all">
          {data.final_url}
        </CardDescription>
      </CardHeader>
      <CardContent className="space-y-4">
        <div className="flex flex-wrap items-center gap-2">
          <Badge
            variant={
              data.http_status >= 200 && data.http_status < 400
                ? "outline"
                : "destructive"
            }
          >
            HTTP {data.http_status}
          </Badge>
          <Badge variant={data.blocked ? "destructive" : "secondary"}>
            {data.blocked ? "BLOCKED" : "NOT BLOCKED"}
          </Badge>
          <Badge variant="secondary">{data.elapsed_ms}ms</Badge>
        </div>

        <div className="grid gap-3 sm:grid-cols-2">
          <InfoItem label="Final URL" value={data.final_url} mono />
          <InfoItem label="User Agent" value={data.user_agent} mono />
          <InfoItem
            label="HTML Length"
            value={`${data.html_length.toLocaleString()} bytes`}
          />
          <InfoItem
            label="Cookies"
            value={`${data.cookies_count} collected`}
          />
        </div>

        {hasExtracted && (
          <div>
            <p className="mb-2 text-xs font-medium uppercase tracking-wider text-muted-foreground">
              Extracted data
            </p>
            <pre className="overflow-auto rounded-lg border bg-muted/50 p-4 text-xs leading-relaxed">
              {JSON.stringify(data.extracted, null, 2)}
            </pre>
          </div>
        )}

        {data.screenshot_base64 && (
          <div>
            <p className="mb-2 text-xs font-medium uppercase tracking-wider text-muted-foreground">
              Screenshot
            </p>
            <img
              src={`data:image/jpeg;base64,${data.screenshot_base64}`}
              alt="Screenshot"
              className="w-full rounded-lg border"
            />
          </div>
        )}

        {data.html && (
          <div>
            <Button
              variant="ghost"
              size="sm"
              onClick={() => setShowHtml(!showHtml)}
              className="gap-1 px-0 text-xs text-muted-foreground"
            >
              {showHtml ? (
                <ChevronDown className="size-3" />
              ) : (
                <ChevronRight className="size-3" />
              )}
              {showHtml ? "Hide" : "Show"} HTML preview
            </Button>
            {showHtml && (
              <pre className="mt-2 max-h-72 overflow-auto rounded-lg border bg-muted/50 p-4 text-xs leading-relaxed">
                {data.html.slice(0, 50000)}
              </pre>
            )}
          </div>
        )}
      </CardContent>
    </Card>
  )
}

function InfoItem({
  label,
  value,
  mono,
}: {
  label: string
  value: string
  mono?: boolean
}) {
  return (
    <div className="rounded-lg border bg-muted/30 p-3">
      <p className="mb-1 text-[11px] font-medium uppercase tracking-wider text-muted-foreground">
        {label}
      </p>
      <p className={`text-sm break-all ${mono ? "font-mono text-xs" : ""}`}>
        {value}
      </p>
    </div>
  )
}
