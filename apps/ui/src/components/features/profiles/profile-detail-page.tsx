import { useCallback, useState } from 'react'

import { BackButton } from '@/components/ui/back-button'
import { Badge } from '@/components/ui/badge'
import { Page, PageHeader } from '@/components/ui/page'
import { Stack } from '@/components/ui/stack'
import { Tabs, TabsContent, TabsList, TabsTrigger } from '@/components/ui/tabs'
import { useHealth } from '@/apis/health'
import { useProfile, useProfileValidation } from '@/apis/profiles'
import { DiagnosticsSummary } from './diagnostics-summary'
import { DiagnosticsTable } from './diagnostics-table'
import { ProfileDetailActions } from './profile-detail-actions'
import { ProfileDetailError, ProfileDetailLoading } from './profile-detail-states'
import { ProfileEditTab } from './profile-edit-tab'
import { ProfileJson } from './profile-json'
import { ProfileOverview } from './profile-overview'
import { VariantsPanel } from './variants-panel'

function UnsavedBadge({ dirty }: { dirty: boolean }) {
  return dirty ? (
    <Badge variant="secondary" className="ml-1">
      Unsaved
    </Badge>
  ) : null
}

export function ProfileDetailPage({ name }: { name: string }) {
  const profile = useProfile(name)
  const validation = useProfileValidation(name)
  const health = useHealth()
  const llmReady = health.data?.llm_configured ?? false
  const diagnostics = validation.data ?? []
  const [editDirty, setEditDirty] = useState(false)
  const [jsonDirty, setJsonDirty] = useState(false)
  const onEditDirty = useCallback((dirty: boolean) => setEditDirty(dirty), [])
  const onJsonDirty = useCallback((dirty: boolean) => setJsonDirty(dirty), [])

  return (
    <Page>
      <Stack gap="xs">
        <BackButton fallback="/profiles" label="All profiles" />
        <PageHeader
          title={name}
          description="Fingerprint profile"
          actions={
            profile.data ? (
              <ProfileDetailActions name={name} profile={profile.data} llmReady={llmReady} />
            ) : null
          }
        />
      </Stack>
      {profile.isPending ? <ProfileDetailLoading /> : null}
      {profile.isError ? <ProfileDetailError name={name} error={profile.error} /> : null}
      {profile.data && validation.data ? <DiagnosticsSummary diagnostics={diagnostics} /> : null}
      {profile.data ? (
        <Tabs defaultValue="overview">
          <TabsList>
            <TabsTrigger value="overview">Overview</TabsTrigger>
            <TabsTrigger value="diagnostics">Diagnostics ({diagnostics.length})</TabsTrigger>
            <TabsTrigger value="edit">
              Edit
              <UnsavedBadge dirty={editDirty} />
            </TabsTrigger>
            <TabsTrigger value="json">
              JSON
              <UnsavedBadge dirty={jsonDirty} />
            </TabsTrigger>
            <TabsTrigger value="variants">Variants</TabsTrigger>
          </TabsList>
          <TabsContent value="overview">
            <ProfileOverview profile={profile.data} />
          </TabsContent>
          <TabsContent value="diagnostics">
            <DiagnosticsTable diagnostics={diagnostics} />
          </TabsContent>
          <TabsContent value="edit" keepMounted>
            <ProfileEditTab name={name} profile={profile.data} onDirtyChange={onEditDirty} />
          </TabsContent>
          <TabsContent value="json" keepMounted>
            <ProfileJson name={name} profile={profile.data} onDirtyChange={onJsonDirty} />
          </TabsContent>
          <TabsContent value="variants">
            <VariantsPanel name={name} profile={profile.data} llmReady={llmReady} />
          </TabsContent>
        </Tabs>
      ) : null}
    </Page>
  )
}
