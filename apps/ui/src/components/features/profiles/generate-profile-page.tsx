import { useState } from 'react'
import { Link } from '@tanstack/react-router'
import { ArrowLeft } from 'lucide-react'
import { toast } from 'sonner'

import { Alert, AlertDescription, AlertTitle } from '@/components/ui/alert'
import { Button } from '@/components/ui/button'
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card'
import { Page, PageHeader } from '@/components/ui/page'
import { Stack } from '@/components/ui/stack'
import { useHealth } from '@/apis/health'
import { useGenerateProfile, type TGenerateProfilePayload } from '@/apis/profiles'
import { toErrorMessage } from '@/libs/http'
import { LLM_MISSING_MESSAGE } from './ai-action-button'
import { GenerateProfileForm } from './generate-profile-form'
import { GeneratedProfilePreview } from './generated-profile-preview'

export function GenerateProfilePage() {
  const health = useHealth()
  const generate = useGenerateProfile()
  const [name, setName] = useState('')
  const llmReady = health.data?.llm_configured ?? false

  async function onGenerate(payload: TGenerateProfilePayload) {
    try {
      await generate.mutateAsync(payload)
      setName(payload.name)
    } catch (error) {
      toast.error(toErrorMessage(error))
    }
  }

  return (
    <Page>
      <PageHeader
        title="New profile"
        description="Generate a coherent fingerprint profile from a persona description."
        actions={
          <Button variant="outline" render={<Link to="/profiles" />}>
            <ArrowLeft />
            Profiles
          </Button>
        }
      />
      <Stack gap="lg">
        {health.data && !llmReady ? (
          <Alert>
            <AlertTitle>AI is not configured</AlertTitle>
            <AlertDescription>{LLM_MISSING_MESSAGE}</AlertDescription>
          </Alert>
        ) : null}
        <Card>
          <CardHeader>
            <CardTitle>Persona</CardTitle>
            <CardDescription>
              The AI fills in every fingerprint surface so the values agree with each other.
            </CardDescription>
          </CardHeader>
          <CardContent>
            <GenerateProfileForm
              llmReady={llmReady}
              pending={generate.isPending}
              onGenerate={onGenerate}
            />
          </CardContent>
        </Card>
        {generate.data && name ? (
          <GeneratedProfilePreview name={name} result={generate.data} />
        ) : null}
      </Stack>
    </Page>
  )
}
