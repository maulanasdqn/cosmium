import { Bot, Cpu, Globe, UserRound } from 'lucide-react'

import { useHealth } from '@/apis/health'
import { useProfileSummaries } from '@/apis/profiles'
import { Badge } from '@/components/ui/badge'
import { Grid } from '@/components/ui/stack'
import { StatusCard } from './status-card'

function profilesWithErrors(errors: (number | null | undefined)[]): number {
  return errors.filter((count) => (count ?? 0) > 0).length
}

export function StatusGrid() {
  const health = useHealth()
  const profiles = useProfileSummaries()
  const data = health.data
  const summaries = profiles.data ?? []
  const broken =
    profilesWithErrors(summaries.map((summary) => summary.errors)) +
    summaries.filter((summary) => summary.load_error).length

  return (
    <Grid columns={4}>
      <StatusCard
        title="Engine"
        icon={<Globe />}
        loading={health.isPending}
        value={health.isError ? 'Offline' : 'Online'}
        badge={
          <Badge variant={health.isError ? 'destructive' : 'secondary'}>
            {health.isError ? 'Unreachable' : (data?.status ?? 'ok')}
          </Badge>
        }
        hint={data ? `Version ${data.version}` : 'Cannot reach the engine API'}
      />
      <StatusCard
        title="Browser binary"
        icon={<Cpu />}
        loading={health.isPending}
        value={data?.binary_found ? 'Ready' : 'Missing'}
        badge={
          <Badge variant={data?.binary_found ? 'secondary' : 'destructive'}>
            {data?.binary_found ? 'Found' : 'Not found'}
          </Badge>
        }
        hint={
          data?.binary_found
            ? 'Patched Chromium is available to launch'
            : 'Set COSMIUM_BINARY or download a prebuilt browser'
        }
      />
      <StatusCard
        title="AI profile tools"
        icon={<Bot />}
        loading={health.isPending}
        value={data?.llm_configured ? 'Enabled' : 'Disabled'}
        badge={
          <Badge variant={data?.llm_configured ? 'secondary' : 'outline'}>
            {data?.llm_configured ? 'Configured' : 'No key'}
          </Badge>
        }
        hint={
          data?.llm_configured
            ? 'Generate, repair, and mutate profiles'
            : 'Configure an LLM key to generate profiles'
        }
      />
      <StatusCard
        title="Profiles"
        icon={<UserRound />}
        loading={profiles.isPending}
        value={summaries.length}
        badge={
          broken > 0 ? (
            <Badge variant="destructive">{broken} with errors</Badge>
          ) : (
            <Badge variant="secondary">All valid</Badge>
          )
        }
        hint="Fingerprint profiles on the engine"
      />
    </Grid>
  )
}
