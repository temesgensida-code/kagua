/**
 * serviceWarmup.svelte.ts
 *
 * NOTE: This file MUST use the .svelte.ts extension so that the Svelte
 * compiler processes it and enables Svelte 5 runes ($state, $effect, etc.).
 * Plain .ts files do not support runes and will throw ReferenceError at runtime.
 *
 * Proactively wakes the Python Parser and SWI-Prolog Reasoner containers on
 * Render Free Tier before the user uploads a document.
 *
 * Strategy:
 *   - Calls GET /warmup on the Rust backend (which itself pings both services)
 *   - Polls every POLL_INTERVAL_MS, up to MAX_ATTEMPTS times
 *   - Exposes a reactive $state object consumable by Svelte components
 */

const RAW_API_BASE = (import.meta.env.VITE_PUBLIC_API_URL as string) || 'http://127.0.0.1:8080';
const API_BASE = RAW_API_BASE.replace(/\/$/, '');

export type WarmupStatus = 'idle' | 'warming' | 'ready' | 'failed';
export type ServiceStatus = 'online' | 'warming' | 'unknown';

export interface WarmupState {
  status: WarmupStatus;
  parser: ServiceStatus;
  reasoner: ServiceStatus;
  attemptCount: number;
}

const MAX_ATTEMPTS     = 6;        // 6 × 12 s = 72 s max warm-up window
const POLL_INTERVAL_MS = 12_000;   // 12 seconds between polls
const REQUEST_TIMEOUT  = 29_000;   // 29 s — just under Render's 30 s boot timeout

// Reactive state — $state works here because this is a .svelte.ts file
export const warmupState = $state<WarmupState>({
  status:       'idle',
  parser:       'unknown',
  reasoner:     'unknown',
  attemptCount: 0,
});

let _warmupRunning = false;

/** Ping /warmup once, update per-service status, return true if both online. */
async function pingWarmup(): Promise<boolean> {
  try {
    const res = await fetch(`${API_BASE}/warmup`, {
      signal: AbortSignal.timeout(REQUEST_TIMEOUT),
    });

    // Accept both 200 (ready) and 503 (still waking) — both carry JSON
    if (!res.ok && res.status !== 503) return false;

    const data = await res.json();
    warmupState.parser   = (data?.services?.parser   ?? 'unknown') as ServiceStatus;
    warmupState.reasoner = (data?.services?.reasoner ?? 'unknown') as ServiceStatus;
    return data?.ready === true;
  } catch {
    // Network error or timeout — services still starting up
    return false;
  }
}

/**
 * Start the warm-up polling loop.
 * Safe to call multiple times — only one loop runs at a time.
 */
export async function warmupServices(): Promise<void> {
  if (_warmupRunning || warmupState.status === 'ready') return;
  _warmupRunning = true;

  warmupState.status       = 'warming';
  warmupState.parser       = 'unknown';
  warmupState.reasoner     = 'unknown';
  warmupState.attemptCount = 0;

  for (let i = 0; i < MAX_ATTEMPTS; i++) {
    warmupState.attemptCount = i + 1;

    const ready = await pingWarmup();
    if (ready) {
      warmupState.status = 'ready';
      _warmupRunning = false;
      return;
    }

    // Wait before next poll (skip wait on last attempt)
    if (i < MAX_ATTEMPTS - 1) {
      await new Promise<void>((r) => setTimeout(r, POLL_INTERVAL_MS));
    }
  }

  // All attempts exhausted — mark failed but do NOT block the user
  warmupState.status = 'failed';
  _warmupRunning = false;
}

/** Reset warm-up state (e.g. on logout or manual retry). */
export function resetWarmup(): void {
  warmupState.status       = 'idle';
  warmupState.parser       = 'unknown';
  warmupState.reasoner     = 'unknown';
  warmupState.attemptCount = 0;
  _warmupRunning           = false;
}
