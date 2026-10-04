import {
  Code,
  Keyboard,
  Link2,
  ListChecks,
  MousePointerClick,
  MoveVertical,
  Timer,
} from 'lucide-react'
import type { LucideIcon } from 'lucide-react'

import type { TStepType } from './types'

export type TStepMeta = {
  type: TStepType
  label: string
  description: string
  icon: LucideIcon
}

export const stepMeta: Record<TStepType, TStepMeta> = {
  click: {
    type: 'click',
    label: 'Click',
    description: 'Click the first element that matches',
    icon: MousePointerClick,
  },
  input: {
    type: 'input',
    label: 'Type text',
    description: 'Type into an input or textarea',
    icon: Keyboard,
  },
  scroll: {
    type: 'scroll',
    label: 'Scroll',
    description: 'Scroll the page to load more content',
    icon: MoveVertical,
  },
  delay: {
    type: 'delay',
    label: 'Wait',
    description: 'Pause before the next step',
    icon: Timer,
  },
  extract: {
    type: 'extract',
    label: 'Extract',
    description: 'Pull text or an attribute into the result',
    icon: ListChecks,
  },
  script: {
    type: 'script',
    label: 'Run script',
    description: 'Evaluate JavaScript and capture what it returns',
    icon: Code,
  },
  follow_urls: {
    type: 'follow_urls',
    label: 'Follow links',
    description: 'Open each matching link and run steps there',
    icon: Link2,
  },
}

export const stepMetaList = Object.values(stepMeta)
