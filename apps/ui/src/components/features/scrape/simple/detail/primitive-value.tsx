import { Badge } from '@/components/ui/badge'
import { ExternalLink } from '@/components/ui/external-link'
import { ImagePreview } from '@/components/ui/code-block'
import { Text } from '@/components/ui/typography'
import { isImageUrl, isUrl } from './values'

export function PrimitiveValue({ value }: { value: string | number | boolean | null }) {
  if (value === null || value === '') {
    return <Text variant="muted">—</Text>
  }
  if (typeof value === 'boolean') {
    return <Badge variant={value ? 'secondary' : 'outline'}>{value ? 'Yes' : 'No'}</Badge>
  }
  if (typeof value === 'number') {
    return <Text weight="medium">{value.toLocaleString()}</Text>
  }
  if (isImageUrl(value)) {
    return <ImagePreview src={value} alt="" loading="lazy" className="max-h-40 w-auto bg-white" />
  }
  if (isUrl(value)) {
    return (
      <ExternalLink href={value} className="font-mono text-xs break-all text-primary">
        {value}
      </ExternalLink>
    )
  }
  return <Text className="whitespace-pre-line break-words">{value}</Text>
}
