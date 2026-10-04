import { createFileRoute } from '@tanstack/react-router'

import { ScrapeResultPage } from '@/components/features/scrape/result'

export const Route = createFileRoute('/_app/scrape_/$id')({
  component: ScrapeResultRoute,
})

function ScrapeResultRoute() {
  const { id } = Route.useParams()
  return <ScrapeResultPage id={id} />
}
