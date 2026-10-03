import type { TScrapeResult } from '@/apis/scrape'
import { Item, ItemContent, ItemDescription, ItemGroup, ItemTitle } from '@/components/ui/item'
import { formatBytes } from '@/components/features/runs'

function DetailRow({ label, value }: { label: string; value: string }) {
  return (
    <Item size="sm">
      <ItemContent>
        <ItemTitle>{label}</ItemTitle>
        <ItemDescription className="font-mono break-all">{value}</ItemDescription>
      </ItemContent>
    </Item>
  )
}

export function ResultDetails({ result }: { result: TScrapeResult }) {
  return (
    <ItemGroup>
      <DetailRow label="Requested URL" value={result.url} />
      <DetailRow label="Final URL" value={result.final_url} />
      <DetailRow label="User agent" value={result.user_agent} />
      <DetailRow label="HTML size" value={formatBytes(result.html_length)} />
      <DetailRow label="Cookies" value={String(result.cookies_count)} />
    </ItemGroup>
  )
}
