import { useEffect, useState } from 'react'

export function useElapsed(running: boolean): number {
  const [elapsed, setElapsed] = useState(0)

  useEffect(() => {
    if (!running) {
      return
    }
    const startedAt = Date.now()
    setElapsed(0)
    const timer = window.setInterval(() => setElapsed(Date.now() - startedAt), 100)
    return () => window.clearInterval(timer)
  }, [running])

  return elapsed
}
