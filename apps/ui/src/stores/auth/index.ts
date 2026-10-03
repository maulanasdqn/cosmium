import { Store, useStore } from '@tanstack/react-store'

const STORAGE_KEY = 'cosmium.apikey'

export type TAuthStatus = 'checking' | 'signed-out' | 'signed-in'

export type TAuthState = {
  apiKey: string | null
  status: TAuthStatus
}

function readKey(): string | null {
  try {
    return localStorage.getItem(STORAGE_KEY)
  } catch {
    return null
  }
}

function writeKey(key: string | null) {
  try {
    if (key) {
      localStorage.setItem(STORAGE_KEY, key)
    } else {
      localStorage.removeItem(STORAGE_KEY)
    }
  } catch {
    return
  }
}

const initialKey = readKey()

export const authStore = new Store<TAuthState>({
  apiKey: initialKey,
  status: initialKey ? 'checking' : 'signed-out',
})

export function signIn(apiKey: string) {
  writeKey(apiKey)
  authStore.setState(() => ({ apiKey, status: 'signed-in' }))
}

export function signOut() {
  writeKey(null)
  authStore.setState(() => ({ apiKey: null, status: 'signed-out' }))
}

export function markSignedIn() {
  authStore.setState((state) => ({ ...state, status: 'signed-in' }))
}

export function useAuthStatus() {
  return useStore(authStore, (state) => state.status)
}
