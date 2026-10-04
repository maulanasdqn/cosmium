import {
  Archive,
  FlaskConical,
  History,
  LayoutDashboard,
  ScanSearch,
  UserRound,
} from 'lucide-react'
import type { LucideIcon } from 'lucide-react'

export type TNavItem = {
  to: '/' | '/profiles' | '/scrape' | '/results' | '/history' | '/tests'
  label: string
  icon: LucideIcon
}

export const navItems: TNavItem[] = [
  { to: '/', label: 'Dashboard', icon: LayoutDashboard },
  { to: '/profiles', label: 'Profiles', icon: UserRound },
  { to: '/scrape', label: 'Scrape', icon: ScanSearch },
  { to: '/results', label: 'Results', icon: Archive },
  { to: '/history', label: 'Run history', icon: History },
  { to: '/tests', label: 'Stealth tests', icon: FlaskConical },
]

export function titleForPath(pathname: string): string {
  const match = [...navItems]
    .sort((a, b) => b.to.length - a.to.length)
    .find((item) => (item.to === '/' ? pathname === '/' : pathname.startsWith(item.to)))
  return match?.label ?? 'Cosmium'
}
