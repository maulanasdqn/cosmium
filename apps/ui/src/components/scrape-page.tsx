import { useState } from "react"
import { ScrapeForm } from "@/components/scrape-form"
import { ResultView } from "@/components/result-view"
import type { ScrapeResponse } from "@/lib/api"

export function ScrapePage() {
  const [result, setResult] = useState<ScrapeResponse | null>(null)

  return (
    <div className="space-y-6">
      <div>
        <h2 className="text-2xl font-bold tracking-tight">Single Scrape</h2>
        <p className="text-sm text-muted-foreground">
          Scrape a single page with the stealth browser engine
        </p>
      </div>
      <ScrapeForm onResult={setResult} />
      {result && <ResultView data={result} />}
    </div>
  )
}
