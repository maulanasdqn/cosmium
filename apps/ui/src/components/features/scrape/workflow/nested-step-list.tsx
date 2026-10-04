import { Empty, EmptyDescription, EmptyHeader, EmptyTitle } from '@/components/ui/empty'
import { Stack } from '@/components/ui/stack'
import { AddStepMenu } from './add-step-menu'
import { StepCard } from './step-card'
import type { TLeafStep } from './types'
import { addStep } from './workflow-store'

export function NestedStepList({
  parentIndex,
  steps,
}: {
  parentIndex: number
  steps: TLeafStep[]
}) {
  return (
    <Stack gap="sm" className="border-l-2 border-border/60 pl-4">
      {steps.length === 0 ? (
        <Empty className="border py-6">
          <EmptyHeader>
            <EmptyTitle>No steps yet</EmptyTitle>
            <EmptyDescription>Add what should happen on each page you open.</EmptyDescription>
          </EmptyHeader>
        </Empty>
      ) : (
        steps.map((step, index) => (
          <StepCard
            key={`${parentIndex}-${index}-${step.type}`}
            step={step}
            path={{ index: parentIndex, child: index }}
            position={index}
            total={steps.length}
            label={`${parentIndex + 1}.${index + 1}`}
          />
        ))
      )}
      <AddStepMenu
        nested
        variant="ghost"
        label="Add nested step"
        onAdd={(type) => addStep(type, parentIndex)}
      />
    </Stack>
  )
}
