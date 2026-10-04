import { Plus } from 'lucide-react'

import { Button } from '@/components/ui/button'
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuTrigger,
} from '@/components/ui/dropdown-menu'
import { Stack } from '@/components/ui/stack'
import { Text } from '@/components/ui/typography'
import { stepMetaList } from './step-meta'
import { isLeafStepType, type TStepType } from './types'

export function AddStepMenu({
  onAdd,
  nested = false,
  label = 'Add step',
  variant = 'outline',
}: {
  onAdd: (type: TStepType) => void
  nested?: boolean
  label?: string
  variant?: 'outline' | 'default' | 'ghost'
}) {
  const options = nested ? stepMetaList.filter((meta) => isLeafStepType(meta.type)) : stepMetaList
  return (
    <DropdownMenu>
      <DropdownMenuTrigger render={<Button variant={variant} size="sm" />}>
        <Plus />
        {label}
      </DropdownMenuTrigger>
      <DropdownMenuContent align="start" className="w-72">
        {options.map((meta) => (
          <DropdownMenuItem key={meta.type} onClick={() => onAdd(meta.type)}>
            <meta.icon />
            <Stack gap="none">
              <Text weight="medium" inline>
                {meta.label}
              </Text>
              <Text variant="small" inline>
                {meta.description}
              </Text>
            </Stack>
          </DropdownMenuItem>
        ))}
      </DropdownMenuContent>
    </DropdownMenu>
  )
}
