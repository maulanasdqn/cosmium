import { useState } from "react"
import { useMutation, useQueryClient } from "@tanstack/react-query"
import { Loader2, Sparkles, Save, Check, AlertTriangle } from "lucide-react"
import { Button } from "@/components/ui/button"
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from "@/components/ui/card"
import { Input } from "@/components/ui/input"
import { Label } from "@/components/ui/label"
import { Textarea } from "@/components/ui/textarea"
import { Badge } from "@/components/ui/badge"
import {
  generateProfile,
  saveProfile,
  type GenerateProfileResponse,
} from "@/lib/api"
import { queryKeys } from "@/lib/query"

export function GenerateProfilePage() {
  const queryClient = useQueryClient()
  const [persona, setPersona] = useState("")
  const [name, setName] = useState("")
  const [result, setResult] = useState<GenerateProfileResponse | null>(null)
  const [saved, setSaved] = useState(false)

  const generateMutation = useMutation({
    mutationFn: generateProfile,
    onSuccess: (data) => {
      setResult(data)
      setSaved(false)
    },
  })

  const saveMutation = useMutation({
    mutationFn: saveProfile,
    onSuccess: () => {
      setSaved(true)
      queryClient.invalidateQueries({ queryKey: queryKeys.profiles })
    },
  })

  function handleGenerate(e: React.FormEvent) {
    e.preventDefault()
    if (generateMutation.isPending || !persona.trim() || !name.trim()) return
    setResult(null)
    setSaved(false)
    generateMutation.mutate({ persona: persona.trim(), name: name.trim() })
  }

  function handleSave() {
    if (!result || saveMutation.isPending) return
    saveMutation.mutate({ name: name.trim(), profile: result.profile })
  }

  const errors = result?.diagnostics.filter((d) => d.severity === "error") ?? []
  const warnings =
    result?.diagnostics.filter((d) => d.severity === "warning") ?? []

  return (
    <div className="space-y-6">
      <div>
        <h2 className="text-2xl font-bold tracking-tight">
          Generate Profile
        </h2>
        <p className="text-sm text-muted-foreground">
          Use AI to generate a realistic browser fingerprint profile
        </p>
      </div>

      <Card>
        <CardHeader>
          <CardTitle className="flex items-center gap-2">
            <Sparkles className="size-5" />
            AI Profile Generator
          </CardTitle>
          <CardDescription>
            Describe the persona you want and the AI will produce a coherent
            browser fingerprint matching real-world hardware and software
            combinations. Powered by DeepSeek.
          </CardDescription>
        </CardHeader>
        <CardContent>
          <form onSubmit={handleGenerate} className="space-y-4">
            <div className="grid gap-4 sm:grid-cols-2">
              <div className="space-y-2">
                <Label htmlFor="profile-name">Profile name</Label>
                <Input
                  id="profile-name"
                  value={name}
                  onChange={(e) => setName(e.target.value)}
                  placeholder="macos_m4_ja-jp"
                  className="font-mono text-sm"
                />
              </div>
              <div className="flex items-end">
                <Button
                  type="submit"
                  disabled={
                    generateMutation.isPending ||
                    !persona.trim() ||
                    !name.trim()
                  }
                  className="w-full sm:w-auto"
                >
                  {generateMutation.isPending && (
                    <Loader2 className="size-4 animate-spin" />
                  )}
                  {generateMutation.isPending ? "Generating…" : "Generate"}
                </Button>
              </div>
            </div>

            <div className="space-y-2">
              <Label htmlFor="persona">Persona description</Label>
              <Textarea
                id="persona"
                value={persona}
                onChange={(e) => setPersona(e.target.value)}
                placeholder={
                  "e.g. A Japanese developer on macOS with an M4 Pro MacBook Pro, " +
                  "16-inch Retina display, Chrome 126, Tokyo timezone, ja-JP locale"
                }
                rows={3}
              />
            </div>
          </form>
        </CardContent>
      </Card>

      {(generateMutation.isError || saveMutation.isError) && (
        <div className="rounded-lg border border-destructive bg-destructive/10 p-3 text-sm text-destructive">
          {(generateMutation.error ?? saveMutation.error)?.message}
        </div>
      )}

      {generateMutation.isPending && (
        <Card>
          <CardContent className="flex items-center justify-center gap-3 py-12">
            <Loader2 className="size-6 animate-spin text-muted-foreground" />
            <p className="text-sm text-muted-foreground">
              DeepSeek is generating your profile — this may take 10–30
              seconds…
            </p>
          </CardContent>
        </Card>
      )}

      {result && (
        <>
          {(errors.length > 0 || warnings.length > 0) && (
            <Card>
              <CardHeader className="pb-3">
                <CardTitle className="text-base">Validation</CardTitle>
              </CardHeader>
              <CardContent className="space-y-2">
                {errors.map((d, i) => (
                  <div
                    key={`e-${i}`}
                    className="flex items-start gap-2 text-sm text-destructive"
                  >
                    <AlertTriangle className="mt-0.5 size-4 shrink-0" />
                    <span>
                      <span className="font-mono text-xs">{d.code}</span>{" "}
                      {d.message}
                    </span>
                  </div>
                ))}
                {warnings.map((d, i) => (
                  <div
                    key={`w-${i}`}
                    className="flex items-start gap-2 text-sm text-yellow-500"
                  >
                    <AlertTriangle className="mt-0.5 size-4 shrink-0" />
                    <span>
                      <span className="font-mono text-xs">{d.code}</span>{" "}
                      {d.message}
                    </span>
                  </div>
                ))}
              </CardContent>
            </Card>
          )}

          <Card>
            <CardHeader>
              <div className="flex items-center justify-between">
                <div>
                  <CardTitle className="text-base">
                    Generated Profile
                  </CardTitle>
                  <CardDescription className="font-mono text-xs">
                    {name}
                  </CardDescription>
                </div>
                <div className="flex items-center gap-2">
                  {errors.length === 0 && (
                    <Badge variant="outline" className="text-green-500">
                      valid
                    </Badge>
                  )}
                  {errors.length > 0 && (
                    <Badge variant="destructive">
                      {errors.length} error{errors.length > 1 ? "s" : ""}
                    </Badge>
                  )}
                  <Button
                    size="sm"
                    onClick={handleSave}
                    disabled={saveMutation.isPending || saved}
                  >
                    {saveMutation.isPending && (
                      <Loader2 className="size-4 animate-spin" />
                    )}
                    {saved && <Check className="size-4" />}
                    {!saveMutation.isPending && !saved && (
                      <Save className="size-4" />
                    )}
                    {saved
                      ? "Saved"
                      : saveMutation.isPending
                        ? "Saving…"
                        : "Save profile"}
                  </Button>
                </div>
              </div>
            </CardHeader>
            <CardContent>
              <pre className="max-h-[500px] overflow-auto rounded-lg border bg-muted/50 p-4 text-xs leading-relaxed">
                {JSON.stringify(result.profile, null, 2)}
              </pre>
            </CardContent>
          </Card>
        </>
      )}
    </div>
  )
}
