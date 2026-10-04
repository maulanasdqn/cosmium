import { Card, CardContent } from '@/components/ui/card'
import { ImagePreview } from '@/components/ui/code-block'
import { ExternalLink } from '@/components/ui/external-link'
import { Grid } from '@/components/ui/stack'
import { Text } from '@/components/ui/typography'
import type { TPageData } from './page-data'

export function ImagesGrid({ images }: { images: NonNullable<TPageData['images']> }) {
  return (
    <Grid columns={4}>
      {images.map((image) => (
        <Card key={image.src} size="sm">
          <CardContent>
            <ExternalLink href={image.src}>
              <ImagePreview
                src={image.src}
                alt={image.alt}
                loading="lazy"
                className="aspect-video bg-muted object-contain"
              />
            </ExternalLink>
            <Text variant="small" truncate className="mt-2">
              {image.alt || 'No alt text'}
            </Text>
          </CardContent>
        </Card>
      ))}
    </Grid>
  )
}
