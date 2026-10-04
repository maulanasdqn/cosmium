import { ArrowDown, ArrowUp, Copy, Trash2 } from 'lucide-react'

import { Button } from '@/components/ui/button'
import { ButtonGroup } from '@/components/ui/button-group'
import { Tooltip, TooltipContent, TooltipTrigger } from '@/components/ui/tooltip'
import { duplicateStep, moveStep, removeStep } from './workflow-store'
import type { TStepPath } from './types'

function IconAction({
  label,
  disabled,
  onClick,
  children,
}: {
  label: string
  disabled?: boolean
  onClick: () => void
  children: React.ReactNode
}) {
  return (
    <Tooltip>
      <TooltipTrigger
        render={
          <Button
            variant="ghost"
            size="icon-sm"
            aria-label={label}
            disabled={disabled}
            onClick={onClick}
          />
        }
      >
        {children}
      </TooltipTrigger>
      <TooltipContent>{label}</TooltipContent>
    </Tooltip>
  )
}

export function StepActions({
  path,
  position,
  total,
}: {
  path: TStepPath
  position: number
  total: number
}) {
  return (
    <ButtonGroup>
      <IconAction label="Move up" disabled={position === 0} onClick={() => moveStep(path, -1)}>
        <ArrowUp />
      </IconAction>
      <IconAction
        label="Move down"
        disabled={position === total - 1}
        onClick={() => moveStep(path, 1)}
      >
        <ArrowDown />
      </IconAction>
      <IconAction label="Duplicate" onClick={() => duplicateStep(path)}>
        <Copy />
      </IconAction>
      <IconAction label="Delete" onClick={() => removeStep(path)}>
        <Trash2 />
      </IconAction>
    </ButtonGroup>
  )
}
