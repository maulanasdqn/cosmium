import { useMemo } from 'react'
import { toast } from 'sonner'

import { useProfileSummaries } from '@/apis/profiles'
import { useFingerprintTest, useStealthTest } from '@/apis/tests'
import type { TSelectOption } from '@/components/ui/form'
import { Page, PageHeader } from '@/components/ui/page'
import { Separator } from '@/components/ui/separator'
import { toErrorMessage } from '@/libs/http'
import { FingerprintResults } from './fingerprint-results'
import { StealthResults } from './stealth-results'
import { TestSection } from './test-section'
import { TestsConfigCard } from './tests-config-card'
import type { TTestAction, TTestsFormValues } from './tests-schema'
import {
  saveFingerprintResult,
  saveStealthResult,
  setActiveProfile,
  useActiveProfile,
  useFingerprintResult,
  useStealthResult,
} from './tests-store'

export function TestsPage({ profile }: { profile?: string }) {
  const summaries = useProfileSummaries()
  const fingerprint = useFingerprintTest()
  const stealth = useStealthTest()
  const activeProfile = useActiveProfile()
  const fingerprintResult = useFingerprintResult(activeProfile)
  const stealthResult = useStealthResult(activeProfile)

  const profileOptions = useMemo<TSelectOption[]>(
    () =>
      (summaries.data ?? [])
        .filter((summary) => !summary.load_error)
        .map((summary) => ({ value: summary.name, label: summary.name })),
    [summaries.data],
  )

  const pendingAction: TTestAction | null = fingerprint.isPending
    ? 'fingerprint'
    : stealth.isPending
      ? 'stealth'
      : null

  function run(action: TTestAction, values: TTestsFormValues) {
    setActiveProfile(values.profile)
    if (action === 'fingerprint') {
      fingerprint.mutate(
        { profile: values.profile, geo_sync: values.geo_sync },
        {
          onSuccess: (result) => {
            saveFingerprintResult(values.profile, result)
            toast.success(`${result.passed} of ${result.passed + result.failed} probes passed`)
          },
          onError: (error) => toast.error(toErrorMessage(error)),
        },
      )
      return
    }
    stealth.mutate(
      {
        profile: values.profile,
        geo_sync: values.geo_sync,
        bot_check_url: values.bot_check_url || null,
      },
      {
        onSuccess: (result) => {
          saveStealthResult(values.profile, result)
          toast.success(`External checks finished for ${values.profile}`)
        },
        onError: (error) => toast.error(toErrorMessage(error)),
      },
    )
  }

  const subject = activeProfile ? ` for ${activeProfile}` : ''

  return (
    <Page>
      <PageHeader
        title="Stealth tests"
        description="Verify that a profile's fingerprint is consistent and holds up against public detection checks."
      />
      <TestsConfigCard
        defaultProfile={profile}
        profileOptions={profileOptions}
        profilesLoading={summaries.isPending}
        pendingAction={pendingAction}
        onRun={run}
      />
      <TestSection
        title={`Fingerprint probes${subject}`}
        description="Around 60 in-browser probes compare what pages can read against the profile, across the main thread, workers, and iframes."
        pending={fingerprint.isPending}
        pendingHint="Fingerprint probes usually take 5–10 seconds."
        error={fingerprint.isError ? toErrorMessage(fingerprint.error) : null}
        emptyTitle="No fingerprint run yet"
        emptyDescription="Choose a profile and run the fingerprint probes."
      >
        {fingerprintResult ? <FingerprintResults result={fingerprintResult} /> : null}
      </TestSection>
      <Separator />
      <TestSection
        title={`External checks${subject}`}
        description="Loads CreepJS, Pixelscan, and BrowserLeaks with the profile and reads their verdicts."
        pending={stealth.isPending}
        pendingHint="External checks usually take 30–60 seconds."
        error={stealth.isError ? toErrorMessage(stealth.error) : null}
        emptyTitle="No external checks yet"
        emptyDescription="Run the external checks to see how public detection sites score the profile."
      >
        {stealthResult ? <StealthResults result={stealthResult} /> : null}
      </TestSection>
    </Page>
  )
}
