import { Store, useStore } from "@tanstack/react-store"

const STORAGE_KEY = "cosmium.apikey"

type AuthState = {
  apiKey: string | null
  status: "loading" | "out" | "in"
}

export const authStore = new Store<AuthState>({
  apiKey: localStorage.getItem(STORAGE_KEY),
  status: localStorage.getItem(STORAGE_KEY) ? "loading" : "out",
})

export function getApiKey(): string | null {
  return authStore.state.apiKey
}

export function signIn(key: string) {
  localStorage.setItem(STORAGE_KEY, key)
  authStore.setState((s) => ({ ...s, apiKey: key, status: "in" as const }))
}

export function signOut() {
  localStorage.removeItem(STORAGE_KEY)
  authStore.setState(() => ({ apiKey: null, status: "out" as const }))
}

export function setAuthStatus(status: AuthState["status"]) {
  authStore.setState((s) => ({ ...s, status }))
}

export function useAuth() {
  return useStore(authStore)
}
