import { createFileRoute } from '@tanstack/react-router'
import { z } from 'zod'

import { ScrapePage } from '@/components/features/scrape'

const scrapeSearchSchema = z.object({
  from: z.string().optional(),
  profile: z.string().optional(),
})

export const Route = createFileRoute('/_app/scrape')({
  validateSearch: scrapeSearchSchema,
  component: ScrapeRoute,
})

function ScrapeRoute() {
  const { from, profile } = Route.useSearch()
  return <ScrapePage fromRun={from} profile={profile} />
}
