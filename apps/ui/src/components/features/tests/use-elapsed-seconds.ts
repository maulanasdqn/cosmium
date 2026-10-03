import { useEffect, useState } from 'react'

export function useElapsedSeconds(running: boolean): number {
  const [startedAt, setStartedAt] = useState<number | null>(null)
  const [now, setNow] = useState(() => Date.now())

  useEffect(() => {
    if (!running) {
      setStartedAt(null)
      return
    }
    const start = Date.now()
    setStartedAt(start)
    setNow(start)
    const timer = window.setInterval(() => setNow(Date.now()), 1000)
    return () => window.clearInterval(timer)
  }, [running])

  return startedAt === null ? 0 : Math.max(0, Math.floor((now - startedAt) / 1000))
}
