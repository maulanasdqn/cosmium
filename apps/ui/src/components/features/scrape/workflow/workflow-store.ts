import { Store, useStore } from '@tanstack/react-store'

import { cloneStep, createStep } from './factories'
import { workflowSchema } from './schema'
import {
  isFollowUrls,
  type TLeafStep,
  type TStepPath,
  type TStepType,
  type TWorkflowStep,
} from './types'

const STORAGE_KEY = 'cosmium.workflow-builder'

export type TWorkflowState = {
  url: string
  profile: string
  steps: TWorkflowStep[]
}

const fallback: TWorkflowState = { url: '', profile: '', steps: [] }

function readState(): TWorkflowState {
  try {
    const raw = localStorage.getItem(STORAGE_KEY)
    if (!raw) {
      return fallback
    }
    const parsed = JSON.parse(raw) as Partial<TWorkflowState>
    const steps = workflowSchema.safeParse(parsed.steps)
    return {
      url: typeof parsed.url === 'string' ? parsed.url : '',
      profile: typeof parsed.profile === 'string' ? parsed.profile : '',
      steps: steps.success ? steps.data : [],
    }
  } catch {
    return fallback
  }
}

export const workflowStore = new Store<TWorkflowState>(readState())

workflowStore.subscribe((state) => {
  try {
    localStorage.setItem(STORAGE_KEY, JSON.stringify(state))
  } catch {
    return
  }
})

function mapSteps(update: (steps: TWorkflowStep[]) => TWorkflowStep[]) {
  workflowStore.setState((state) => ({ ...state, steps: update([...state.steps]) }))
}

function withChildren(
  steps: TWorkflowStep[],
  index: number,
  update: (children: TLeafStep[]) => TLeafStep[],
): TWorkflowStep[] {
  const parent = steps[index]
  if (!parent || !isFollowUrls(parent)) {
    return steps
  }
  steps[index] = { ...parent, workflow: update([...parent.workflow]) }
  return steps
}

function move<TItem>(items: TItem[], from: number, delta: number): TItem[] {
  const to = from + delta
  const item = items[from]
  if (!item || to < 0 || to >= items.length) {
    return items
  }
  items.splice(from, 1)
  items.splice(to, 0, item)
  return items
}

export function setTarget(patch: Partial<Pick<TWorkflowState, 'url' | 'profile'>>) {
  workflowStore.setState((state) => ({ ...state, ...patch }))
}

export function setSteps(steps: TWorkflowStep[]) {
  workflowStore.setState((state) => ({ ...state, steps }))
}

export function addStep(type: TStepType, parentIndex?: number) {
  mapSteps((steps) => {
    if (parentIndex === undefined) {
      steps.push(createStep(type))
      return steps
    }
    return withChildren(steps, parentIndex, (children) => {
      children.push(createStep(type) as TLeafStep)
      return children
    })
  })
}

export function updateStep(path: TStepPath, patch: Partial<TWorkflowStep>) {
  mapSteps((steps) => {
    if (path.child === undefined) {
      const current = steps[path.index]
      if (current) {
        steps[path.index] = { ...current, ...patch } as TWorkflowStep
      }
      return steps
    }
    return withChildren(steps, path.index, (children) => {
      const current = children[path.child ?? 0]
      if (current) {
        children[path.child ?? 0] = { ...current, ...patch } as TLeafStep
      }
      return children
    })
  })
}

export function removeStep(path: TStepPath) {
  mapSteps((steps) => {
    if (path.child === undefined) {
      steps.splice(path.index, 1)
      return steps
    }
    return withChildren(steps, path.index, (children) => {
      children.splice(path.child ?? 0, 1)
      return children
    })
  })
}

export function duplicateStep(path: TStepPath) {
  mapSteps((steps) => {
    if (path.child === undefined) {
      const current = steps[path.index]
      if (current) {
        steps.splice(path.index + 1, 0, cloneStep(current))
      }
      return steps
    }
    return withChildren(steps, path.index, (children) => {
      const current = children[path.child ?? 0]
      if (current) {
        children.splice((path.child ?? 0) + 1, 0, cloneStep(current))
      }
      return children
    })
  })
}

export function moveStep(path: TStepPath, delta: number) {
  mapSteps((steps) => {
    if (path.child === undefined) {
      return move(steps, path.index, delta)
    }
    return withChildren(steps, path.index, (children) => move(children, path.child ?? 0, delta))
  })
}

export function useWorkflowState() {
  return useStore(workflowStore, (state) => state)
}

export function useWorkflowSteps() {
  return useStore(workflowStore, (state) => state.steps)
}
