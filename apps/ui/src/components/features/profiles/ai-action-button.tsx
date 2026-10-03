import * as React from 'react'

import { Button } from '@/components/ui/button'
import { Spinner } from '@/components/ui/spinner'
import { Tooltip, TooltipContent, TooltipTrigger } from '@/components/ui/tooltip'
import { Text } from '@/components/ui/typography'

export const LLM_MISSING_MESSAGE =
  'AI features need an LLM. Start the server with DEEPSEEK_API_KEY set to enable them.'

export function AiActionButton({
  llmReady,
  pending = false,
  variant = 'outline',
  onClick,
  children,
}: {
  llmReady: boolean
  pending?: boolean
  variant?: 'default' | 'outline' | 'secondary'
  onClick?: () => void
  children: React.ReactNode
}) {
  const button = (
    <Button variant={variant} disabled={!llmReady || pending} onClick={onClick}>
      {pending ? <Spinner /> : null}
      {children}
    </Button>
  )
  if (llmReady) {
    return button
  }
  return (
    <Tooltip>
      <TooltipTrigger render={<Text inline tabIndex={0} className="inline-flex" />}>
        {button}
      </TooltipTrigger>
      <TooltipContent>{LLM_MISSING_MESSAGE}</TooltipContent>
    </Tooltip>
  )
}
