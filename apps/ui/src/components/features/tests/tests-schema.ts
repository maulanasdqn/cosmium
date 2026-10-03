import { z } from 'zod'

export const testsFormSchema = z.object({
  profile: z.string().min(1, 'Choose a profile to test'),
  geo_sync: z.boolean(),
  bot_check_url: z.union([z.literal(''), z.url('Enter a full URL, including https://')]),
})

export type TTestsFormValues = z.infer<typeof testsFormSchema>

export type TTestAction = 'fingerprint' | 'stealth'

export type TTestsSubmitMeta = {
  action: TTestAction
}
