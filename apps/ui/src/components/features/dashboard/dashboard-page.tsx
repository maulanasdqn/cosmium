import { Page, PageHeader } from '@/components/ui/page'
import { Grid } from '@/components/ui/stack'
import { QuickActions } from './quick-actions'
import { RecentRunsCard } from './recent-runs-card'
import { RunStatsCard } from './run-stats-card'
import { StatusGrid } from './status-grid'

export function DashboardPage() {
  return (
    <Page>
      <PageHeader
        title="Dashboard"
        description="Engine health, profiles, and recent scraping activity."
        actions={<QuickActions />}
      />
      <StatusGrid />
      <Grid columns={2}>
        <RunStatsCard />
        <RecentRunsCard />
      </Grid>
    </Page>
  )
}
