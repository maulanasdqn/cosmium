import { Link } from '@tanstack/react-router'
import { Plus, RotateCw, Sparkles } from 'lucide-react'

import type { TStoredResult } from '@/apis/results'
import { Button } from '@/components/ui/button'
import { Spinner } from '@/components/ui/spinner'
import { useRunScrape } from '../use-run-scrape'
import { DeleteResultButton } from './delete-result-button'

export function ResultActions({
  stored,
  showFormat,
  canFormat,
  onFormat,
}: {
  stored: TStoredResult
  showFormat: boolean
  canFormat: boolean
  onFormat: () => void
}) {
  const scrape = useRunScrape()
  return (
    <>
      {showFormat ? (
        <Button variant="outline" size="sm" disabled={!canFormat} onClick={onFormat}>
          <Sparkles />
          Format with AI
        </Button>
      ) : null}
      <Button
        variant="outline"
        size="sm"
        disabled={scrape.pending}
        onClick={() => void scrape.run(stored.payload, stored.ai_request)}
      >
        {scrape.pending ? <Spinner /> : <RotateCw />}
        {scrape.pending ? 'Scraping…' : 'Re-run'}
      </Button>
      <Button size="sm" render={<Link to="/scrape" />}>
        <Plus />
        New scrape
      </Button>
      <DeleteResultButton id={stored.id} />
    </>
  )
}
