import { authStore } from './authStore.svelte';

const RAW_API_BASE = (import.meta.env.VITE_PUBLIC_API_URL as string) || 'http://127.0.0.1:8080';
const API_BASE = RAW_API_BASE.replace(/\/$/, '');

// ── Types ─────────────────────────────────────────────────────────────────────

export interface AuthResponse {
  access_token: string;
  refresh_token: string;
  token_type: string;
  user: {
    id: string;
    email: string;
    is_verified: boolean;
  };
}

export interface ApiError {
  error: string;
  message: string;
}

// ── Internal fetch helper with automatic token refresh ─────────────────────────

async function apiFetch(
  path: string,
  options: RequestInit = {},
  retry = true
): Promise<Response> {
  const headers: Record<string, string> = {
    'Content-Type': 'application/json',
    ...(options.headers as Record<string, string> || {}),
  };

  if (authStore.accessToken) {
    headers['Authorization'] = `Bearer ${authStore.accessToken}`;
  }

  const res = await fetch(`${API_BASE}${path}`, { ...options, headers });

  // 401 → try silent token refresh once
  if (res.status === 401 && retry && authStore.refreshToken) {
    const refreshed = await silentRefresh();
    if (refreshed) {
      return apiFetch(path, options, false);
    }
  }

  return res;
}

async function silentRefresh(): Promise<boolean> {
  if (!authStore.refreshToken) return false;
  try {
    const res = await fetch(`${API_BASE}/auth/refresh`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ refresh_token: authStore.refreshToken }),
    });
    if (!res.ok) {
      authStore.clearSession();
      return false;
    }
    const data = await res.json();
    authStore.setSession(data.access_token, data.refresh_token, authStore.user!);
    return true;
  } catch {
    authStore.clearSession();
    return false;
  }
}

async function parseApiError(res: Response): Promise<string> {
  try {
    const data: ApiError = await res.json();
    return data.message || `Request failed (${res.status})`;
  } catch {
    return `Request failed (${res.status})`;
  }
}

// ── Public API ────────────────────────────────────────────────────────────────

export async function register(email: string, password: string): Promise<void> {
  const res = await fetch(`${API_BASE}/auth/register`, {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify({ email, password }),
  });
  if (!res.ok) throw new Error(await parseApiError(res));
}

export async function login(
  email: string,
  password: string,
  rememberMe = false
): Promise<AuthResponse> {
  const res = await fetch(`${API_BASE}/auth/login`, {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify({ email, password, remember_me: rememberMe }),
  });
  if (!res.ok) throw new Error(await parseApiError(res));
  const data: AuthResponse = await res.json();
  authStore.setSession(data.access_token, data.refresh_token, data.user);
  return data;
}

export async function logout(): Promise<void> {
  if (authStore.refreshToken) {
    await fetch(`${API_BASE}/auth/logout`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ refresh_token: authStore.refreshToken }),
    }).catch(() => {});
  }
  authStore.clearSession();
}

export async function verifyEmail(token: string): Promise<void> {
  const res = await fetch(`${API_BASE}/auth/verify-email`, {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify({ token }),
  });
  if (!res.ok) throw new Error(await parseApiError(res));
}

export async function resendVerification(email: string): Promise<void> {
  const res = await fetch(`${API_BASE}/auth/resend-verification`, {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify({ email }),
  });
  if (!res.ok) throw new Error(await parseApiError(res));
}

export async function forgotPassword(email: string): Promise<void> {
  const res = await fetch(`${API_BASE}/auth/forgot-password`, {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify({ email }),
  });
  if (!res.ok) throw new Error(await parseApiError(res));
}

export async function resetPassword(token: string, newPassword: string): Promise<void> {
  const res = await fetch(`${API_BASE}/auth/reset-password`, {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify({ token, new_password: newPassword }),
  });
  if (!res.ok) throw new Error(await parseApiError(res));
}

export async function getMe() {
  const res = await apiFetch('/auth/me');
  if (!res.ok) throw new Error(await parseApiError(res));
  return res.json();
}
