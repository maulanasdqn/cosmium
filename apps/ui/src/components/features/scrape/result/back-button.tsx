import { useCanGoBack, useNavigate, useRouter } from '@tanstack/react-router'
import { ArrowLeft } from 'lucide-react'

import { Button } from '@/components/ui/button'

export function BackButton() {
  const router = useRouter()
  const canGoBack = useCanGoBack()
  const navigate = useNavigate()
  return (
    <Button
      variant="ghost"
      size="sm"
      className="-ml-2 self-start text-muted-foreground"
      onClick={() => (canGoBack ? router.history.back() : void navigate({ to: '/results' }))}
    >
      <ArrowLeft />
      {canGoBack ? 'Back' : 'All results'}
    </Button>
  )
}
