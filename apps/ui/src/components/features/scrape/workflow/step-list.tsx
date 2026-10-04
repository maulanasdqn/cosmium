import { Workflow } from 'lucide-react'

import {
  Empty,
  EmptyContent,
  EmptyDescription,
  EmptyHeader,
  EmptyMedia,
  EmptyTitle,
} from '@/components/ui/empty'
import { Stack } from '@/components/ui/stack'
import { AddStepMenu } from './add-step-menu'
import { StepCard } from './step-card'
import type { TWorkflowStep } from './types'
import { addStep } from './workflow-store'

export function StepList({ steps }: { steps: TWorkflowStep[] }) {
  if (steps.length === 0) {
    return (
      <Empty className="border">
        <EmptyHeader>
          <EmptyMedia variant="icon">
            <Workflow />
          </EmptyMedia>
          <EmptyTitle>No steps yet</EmptyTitle>
          <EmptyDescription>
            Build the path a person would take: click, type, scroll, then extract what you need.
          </EmptyDescription>
        </EmptyHeader>
        <EmptyContent>
          <AddStepMenu
            variant="default"
            label="Add the first step"
            onAdd={(type) => addStep(type)}
          />
        </EmptyContent>
      </Empty>
    )
  }

  return (
    <Stack gap="md">
      {steps.map((step, index) => (
        <StepCard
          key={`${index}-${step.type}`}
          step={step}
          path={{ index }}
          position={index}
          total={steps.length}
          label={String(index + 1)}
        />
      ))}
      <AddStepMenu onAdd={(type) => addStep(type)} />
    </Stack>
  )
}
