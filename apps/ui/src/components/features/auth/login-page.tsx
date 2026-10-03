import { Globe } from 'lucide-react'

import { BrandMark } from '@/components/ui/brand-mark'
import {
  Card,
  CardContent,
  CardDescription,
  CardFooter,
  CardHeader,
  CardTitle,
} from '@/components/ui/card'
import { CenteredScreen } from '@/components/ui/page'
import { Stack } from '@/components/ui/stack'
import { InlineCode, Text } from '@/components/ui/typography'
import { LoginForm } from './login-form'

export function LoginPage() {
  return (
    <CenteredScreen>
      <Stack gap="lg" className="w-full max-w-sm">
        <BrandMark
          icon={<Globe />}
          title="Cosmium"
          subtitle="Stealth browser platform"
          className="self-center"
        />
        <Card>
          <CardHeader>
            <CardTitle>Sign in</CardTitle>
            <CardDescription>Enter the API key of your Cosmium engine.</CardDescription>
          </CardHeader>
          <CardContent>
            <LoginForm />
          </CardContent>
          <CardFooter>
            <Text variant="small">
              The key is the value passed to <InlineCode>cosmium serve --api-key</InlineCode>.
            </Text>
          </CardFooter>
        </Card>
      </Stack>
    </CenteredScreen>
  )
}
