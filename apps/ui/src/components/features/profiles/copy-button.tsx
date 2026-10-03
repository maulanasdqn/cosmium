import { useState } from 'react'
import { Check, Copy } from 'lucide-react'
import { toast } from 'sonner'

import { Button } from '@/components/ui/button'

export function CopyButton({ value, label = 'Copy' }: { value: string; label?: string }) {
  const [copied, setCopied] = useState(false)

  function copy() {
    navigator.clipboard
      .writeText(value)
      .then(() => {
        setCopied(true)
        window.setTimeout(() => setCopied(false), 1500)
      })
      .catch(() => toast.error('Clipboard is not available'))
  }

  return (
    <Button variant="outline" size="sm" onClick={copy}>
      {copied ? <Check /> : <Copy />}
      {copied ? 'Copied' : label}
    </Button>
  )
}
