import { JsonBlock } from '@/components/ui/json-block'
import { Stack } from '@/components/ui/stack'
import { Text } from '@/components/ui/typography'
import type { TProfile } from '@/apis/profiles'
import { CopyButton } from './copy-button'

export function ProfileJson({
  profile,
  maxHeight = 'max-h-[32rem]',
}: {
  profile: TProfile
  maxHeight?: string
}) {
  const json = JSON.stringify(profile, null, 2)
  return (
    <Stack gap="sm">
      <Stack direction="row" justify="between" align="center">
        <Text variant="muted">{json.split('\n').length} lines</Text>
        <CopyButton value={json} label="Copy JSON" />
      </Stack>
      <JsonBlock data={profile} maxHeight={maxHeight} />
    </Stack>
  )
}
