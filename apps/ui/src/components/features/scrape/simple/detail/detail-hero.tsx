import { Badge } from '@/components/ui/badge'
import { Card, CardContent } from '@/components/ui/card'
import { Grid, Stack } from '@/components/ui/stack'
import { Heading, Text } from '@/components/ui/typography'
import type { THero } from './hero-fields'
import { ImageGallery } from './image-gallery'
import { RatingStars } from './rating-stars'
import { labelOf, type TRecord } from './values'

function flagBadges(record: TRecord): string[] {
  return Object.entries(record)
    .filter(([, value]) => value === true)
    .map(([key]) => labelOf(key))
    .slice(0, 6)
}

export function DetailHero({ hero, record }: { hero: THero; record: TRecord }) {
  const flags = flagBadges(record)
  const content = (
    <Stack gap="md">
      {hero.brand ? <Badge variant="secondary">{hero.brand}</Badge> : null}
      {hero.title ? <Heading level={2}>{hero.title}</Heading> : null}
      {hero.ratingScore === null ? null : (
        <RatingStars score={hero.ratingScore} count={hero.ratingCount} />
      )}
      {hero.price ? (
        <Text className="text-3xl font-semibold tracking-tight">{hero.price}</Text>
      ) : null}
      {flags.length > 0 ? (
        <Stack direction="row" gap="sm" wrap>
          {flags.map((flag) => (
            <Badge key={flag} variant="outline">
              {flag}
            </Badge>
          ))}
        </Stack>
      ) : null}
    </Stack>
  )
  return (
    <Card>
      <CardContent>
        {hero.images.length > 0 ? (
          <Grid columns={2} gap="lg" className="md:grid-cols-[minmax(0,2fr)_minmax(0,3fr)]">
            <ImageGallery images={hero.images} alt={hero.title ?? 'Scraped item'} />
            {content}
          </Grid>
        ) : (
          content
        )}
      </CardContent>
    </Card>
  )
}
