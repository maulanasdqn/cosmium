import { Star } from 'lucide-react'

import { Stack } from '@/components/ui/stack'
import { Text } from '@/components/ui/typography'
import { cn } from '@/libs/utils'

export function RatingStars({ score, count }: { score: number; count: number | null }) {
  const rounded = Math.round(Math.min(Math.max(score, 0), 5))
  return (
    <Stack direction="row" align="center" gap="sm">
      <Stack direction="row" gap="none" aria-label={`${score} out of 5`}>
        {[1, 2, 3, 4, 5].map((index) => (
          <Star
            key={index}
            className={cn(
              'size-4',
              index <= rounded ? 'fill-amber-400 text-amber-400' : 'text-muted-foreground/40',
            )}
          />
        ))}
      </Stack>
      <Text weight="medium" inline>
        {score.toFixed(1)}
      </Text>
      {count === null ? null : (
        <Text variant="muted" inline>
          {count.toLocaleString()} ratings
        </Text>
      )}
    </Stack>
  )
}
