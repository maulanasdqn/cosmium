import { Link } from '@tanstack/react-router'
import { FlaskConical, ScanSearch, UserRound } from 'lucide-react'

import { Button } from '@/components/ui/button'

export function QuickActions() {
  return (
    <>
      <Button variant="outline" render={<Link to="/profiles" />}>
        <UserRound />
        Profiles
      </Button>
      <Button variant="outline" render={<Link to="/tests" />}>
        <FlaskConical />
        Stealth tests
      </Button>
      <Button render={<Link to="/scrape" />}>
        <ScanSearch />
        New scrape
      </Button>
    </>
  )
}
