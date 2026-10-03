import { z } from 'zod'

export const profileNameSchema = z
  .string()
  .trim()
  .min(1, 'Name is required')
  .max(64, 'Keep the name under 64 characters')
  .regex(/^[A-Za-z0-9_-]+$/, 'Use letters, numbers, dashes, and underscores only')
