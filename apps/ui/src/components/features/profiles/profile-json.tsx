import { useState } from 'react'
import { ListChecks, RotateCcw, Save } from 'lucide-react'

import { Alert, AlertDescription, AlertTitle } from '@/components/ui/alert'
import { Button } from '@/components/ui/button'
import { Spinner } from '@/components/ui/spinner'
import { Stack } from '@/components/ui/stack'
import { Textarea } from '@/components/ui/textarea'
import { Text } from '@/components/ui/typography'
import { useValidateDraft, type TDiagnostic, type TProfile } from '@/apis/profiles'
import { CopyButton } from './copy-button'
import { DiagnosticsTable } from './diagnostics-table'
import { IncoherentSaveDialog } from './incoherent-save-dialog'
import { useJsonDraft } from './use-json-draft'
import { useProfileSave } from './use-profile-save'

export function ProfileJson({
  name,
  profile,
  onDirtyChange,
}: {
  name: string
  profile: TProfile
  onDirtyChange?: (dirty: boolean) => void
}) {
  const draft = useJsonDraft(profile, onDirtyChange)
  const validate = useValidateDraft()
  const saver = useProfileSave(name)
  const [checked, setChecked] = useState<TDiagnostic[] | null>(null)

  function runValidate() {
    if (!draft.parsed) {
      return
    }
    validate.mutate(draft.parsed, { onSuccess: setChecked })
  }

  return (
    <Stack gap="sm">
      <Stack direction="row" justify="between" align="center" wrap gap="sm">
        <Text variant="muted">
          {draft.text.split('\n').length} lines{draft.dirty ? ' · unsaved changes' : ''}
        </Text>
        <Stack direction="row" gap="sm" wrap>
          <CopyButton value={draft.text} label="Copy JSON" />
          <Button variant="outline" size="sm" disabled={!draft.dirty} onClick={draft.revert}>
            <RotateCcw />
            Revert
          </Button>
          <Button
            variant="outline"
            size="sm"
            disabled={!draft.parsed || validate.isPending}
            onClick={runValidate}
          >
            {validate.isPending ? <Spinner /> : <ListChecks />}
            Validate
          </Button>
          <Button
            size="sm"
            disabled={!draft.parsed || !draft.dirty || saver.busy}
            onClick={() => (draft.parsed ? void saver.requestSave(draft.parsed) : null)}
          >
            {saver.busy ? <Spinner /> : <Save />}
            Save changes
          </Button>
        </Stack>
      </Stack>
      <Textarea
        value={draft.text}
        rows={24}
        spellCheck={false}
        aria-label={`JSON source of ${name}`}
        aria-invalid={draft.parseError !== null}
        className="font-mono text-xs"
        onChange={(event) => {
          setChecked(null)
          draft.setText(event.target.value)
        }}
      />
      {draft.parseError ? (
        <Alert variant="destructive">
          <AlertTitle>This is not valid JSON</AlertTitle>
          <AlertDescription>{draft.parseError}</AlertDescription>
        </Alert>
      ) : null}
      {checked ? <DiagnosticsTable diagnostics={checked} /> : null}
      <IncoherentSaveDialog
        pending={saver.pending}
        busy={saver.busy}
        onCancel={saver.dismiss}
        onConfirm={saver.saveAnyway}
      />
    </Stack>
  )
}
