import { z } from 'zod'

export const loginSchema = z.object({
  apiKey: z.string().trim().min(1, 'Enter your API key'),
})

export type TLoginValues = z.infer<typeof loginSchema>
