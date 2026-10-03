import { Page, PageHeader } from '@/components/ui/page'
import { Tabs, TabsContent, TabsList, TabsTrigger } from '@/components/ui/tabs'
import { useHealth } from '@/apis/health'
import { useProfile, useProfileValidation } from '@/apis/profiles'
import { DiagnosticsSummary } from './diagnostics-summary'
import { DiagnosticsTable } from './diagnostics-table'
import { ProfileDetailActions } from './profile-detail-actions'
import { ProfileDetailError, ProfileDetailLoading } from './profile-detail-states'
import { ProfileJson } from './profile-json'
import { ProfileOverview } from './profile-overview'
import { VariantsPanel } from './variants-panel'

export function ProfileDetailPage({ name }: { name: string }) {
  const profile = useProfile(name)
  const validation = useProfileValidation(name)
  const health = useHealth()
  const llmReady = health.data?.llm_configured ?? false
  const diagnostics = validation.data ?? []

  return (
    <Page>
      <PageHeader
        title={name}
        description="Fingerprint profile"
        actions={profile.data ? <ProfileDetailActions name={name} llmReady={llmReady} /> : null}
      />
      {profile.isPending ? <ProfileDetailLoading /> : null}
      {profile.isError ? <ProfileDetailError name={name} error={profile.error} /> : null}
      {profile.data && validation.data ? <DiagnosticsSummary diagnostics={diagnostics} /> : null}
      {profile.data ? (
        <Tabs defaultValue="overview">
          <TabsList>
            <TabsTrigger value="overview">Overview</TabsTrigger>
            <TabsTrigger value="diagnostics">Diagnostics ({diagnostics.length})</TabsTrigger>
            <TabsTrigger value="json">JSON</TabsTrigger>
            <TabsTrigger value="variants">Variants</TabsTrigger>
          </TabsList>
          <TabsContent value="overview">
            <ProfileOverview profile={profile.data} />
          </TabsContent>
          <TabsContent value="diagnostics">
            <DiagnosticsTable diagnostics={diagnostics} />
          </TabsContent>
          <TabsContent value="json">
            <ProfileJson profile={profile.data} />
          </TabsContent>
          <TabsContent value="variants">
            <VariantsPanel name={name} profile={profile.data} llmReady={llmReady} />
          </TabsContent>
        </Tabs>
      ) : null}
    </Page>
  )
}
