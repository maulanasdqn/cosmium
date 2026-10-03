import { Grid } from '@/components/ui/stack'
import type { TProfile } from '@/apis/profiles'
import { FactCard } from './fact-card'
import {
  clientHintFacts,
  gpuFacts,
  hardwareFacts,
  identityFacts,
  localeFacts,
  mediaFacts,
  readProfileFacts,
  screenFacts,
} from './profile-facts'

export function ProfileOverview({ profile }: { profile: TProfile }) {
  const facts = readProfileFacts(profile)
  return (
    <Grid columns={2}>
      <FactCard title="Identity" facts={identityFacts(facts)} />
      <FactCard title="Client hints" facts={clientHintFacts(facts)} />
      <FactCard title="Hardware" facts={hardwareFacts(facts)} />
      <FactCard title="GPU" facts={gpuFacts(facts)} />
      <FactCard title="Screen" facts={screenFacts(facts)} />
      <FactCard title="Locale" facts={localeFacts(facts)} />
      <FactCard title="Fonts and voices" facts={mediaFacts(facts)} />
    </Grid>
  )
}
