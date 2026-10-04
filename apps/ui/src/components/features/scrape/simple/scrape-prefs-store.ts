import { Store, useStore } from '@tanstack/react-store'

import { isPreset, type TPreset } from './presets'

const STORAGE_KEY = 'cosmium.scrape-prefs'

export type TScrapePrefs = {
  profile: string
  presets: TPreset[]
  aiFormat: boolean
  proxy: string
  geoSync: boolean
}

const fallback: TScrapePrefs = {
  profile: '',
  presets: ['summary', 'text', 'links'],
  aiFormat: true,
  proxy: '',
  geoSync: false,
}

function readPrefs(): TScrapePrefs {
  try {
    const raw = localStorage.getItem(STORAGE_KEY)
    if (!raw) {
      return fallback
    }
    const parsed = JSON.parse(raw) as Partial<TScrapePrefs>
    return {
      profile: typeof parsed.profile === 'string' ? parsed.profile : '',
      presets: Array.isArray(parsed.presets) ? parsed.presets.filter(isPreset) : fallback.presets,
      aiFormat: typeof parsed.aiFormat === 'boolean' ? parsed.aiFormat : fallback.aiFormat,
      proxy: typeof parsed.proxy === 'string' ? parsed.proxy : '',
      geoSync: typeof parsed.geoSync === 'boolean' ? parsed.geoSync : fallback.geoSync,
    }
  } catch {
    return fallback
  }
}

export const scrapePrefsStore = new Store<TScrapePrefs>(readPrefs())

scrapePrefsStore.subscribe((prefs) => {
  try {
    localStorage.setItem(STORAGE_KEY, JSON.stringify(prefs))
  } catch {
    return
  }
})

export function rememberPrefs(prefs: TScrapePrefs) {
  scrapePrefsStore.setState(() => prefs)
}

export function useScrapePrefs() {
  return useStore(scrapePrefsStore, (state) => state)
}
