import { useState } from 'react'

import { Button } from '@/components/ui/button'
import { ImagePreview } from '@/components/ui/code-block'
import { Stack } from '@/components/ui/stack'
import { cn } from '@/libs/utils'

export function ImageGallery({ images, alt }: { images: string[]; alt: string }) {
  const [active, setActive] = useState(0)
  const current = images[active] ?? images[0]
  if (!current) {
    return null
  }
  return (
    <Stack gap="sm">
      <ImagePreview src={current} alt={alt} className="aspect-square bg-white object-contain p-2" />
      {images.length > 1 ? (
        <Stack direction="row" gap="sm" wrap>
          {images.slice(0, 8).map((src, index) => (
            <Button
              key={src}
              variant="outline"
              className={cn(
                'size-14 p-1',
                index === active && 'border-primary ring-2 ring-primary/40',
              )}
              aria-label={`Show image ${index + 1}`}
              onClick={() => setActive(index)}
            >
              <ImagePreview
                src={src}
                alt=""
                className="size-full border-0 bg-white object-contain"
              />
            </Button>
          ))}
        </Stack>
      ) : null}
    </Stack>
  )
}
