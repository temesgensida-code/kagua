// Auth store — Svelte 5 rune-based session management
// Stores access token in memory, refresh token in localStorage

const ACCESS_TOKEN_KEY = 'kagua_access_token';
const REFRESH_TOKEN_KEY = 'kagua_refresh_token';
const USER_KEY = 'kagua_user';

export interface AuthUser {
  id: string;
  email: string;
  is_verified: boolean;
}

// ── Reactive state ────────────────────────────────────────────────────────────
let accessToken = $state<string | null>(null);
let refreshToken = $state<string | null>(null);
let user = $state<AuthUser | null>(null);
let isLoading = $state(true); // true during initial hydration

// ── Hydrate from localStorage on module init ──────────────────────────────────
if (typeof window !== 'undefined') {
  try {
    const storedAccess = localStorage.getItem(ACCESS_TOKEN_KEY);
    const storedRefresh = localStorage.getItem(REFRESH_TOKEN_KEY);
    const storedUser = localStorage.getItem(USER_KEY);
    if (storedAccess) accessToken = storedAccess;
    if (storedRefresh) refreshToken = storedRefresh;
    if (storedUser) user = JSON.parse(storedUser);
  } catch {
    // ignore parse errors
  }
  isLoading = false;
}

function persist() {
  if (typeof window === 'undefined') return;
  if (accessToken) localStorage.setItem(ACCESS_TOKEN_KEY, accessToken);
  else localStorage.removeItem(ACCESS_TOKEN_KEY);
  if (refreshToken) localStorage.setItem(REFRESH_TOKEN_KEY, refreshToken);
  else localStorage.removeItem(REFRESH_TOKEN_KEY);
  if (user) localStorage.setItem(USER_KEY, JSON.stringify(user));
  else localStorage.removeItem(USER_KEY);
}

export const authStore = {
  // ── Getters ──────────────────────────────────────────────────────────────
  get accessToken() { return accessToken; },
  get refreshToken() { return refreshToken; },
  get user() { return user; },
  get isLoading() { return isLoading; },
  get isAuthenticated() { return !!accessToken && !!user; },

  // ── Actions ───────────────────────────────────────────────────────────────
  setSession(at: string, rt: string, u: AuthUser) {
    accessToken = at;
    refreshToken = rt;
    user = u;
    persist();
  },

  setAccessToken(at: string) {
    accessToken = at;
    if (typeof window !== 'undefined') {
      localStorage.setItem(ACCESS_TOKEN_KEY, at);
    }
  },

  clearSession() {
    accessToken = null;
    refreshToken = null;
    user = null;
    persist();
  },

  setLoading(val: boolean) {
    isLoading = val;
  }
};
