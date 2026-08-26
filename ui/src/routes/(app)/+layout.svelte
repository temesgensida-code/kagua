<script lang="ts">
  import { onMount } from 'svelte';
  import favicon from '$lib/assets/favicon.png';
  import '../../app.css';
  import { authStore } from '$lib/services/authStore.svelte';
  import { logout, silentRefresh } from '$lib/services/authApi';
  import { warmupServices, resetWarmup } from '$lib/services/serviceWarmup.svelte';
  import ServiceWarmupBanner from '$lib/components/ServiceWarmupBanner.svelte';
  import { goto } from '$app/navigation';

  const RAW_API_BASE = (import.meta.env.VITE_PUBLIC_API_URL as string) || 'http://127.0.0.1:8080';
  const API_BASE = RAW_API_BASE.replace(/\/$/, '');

  let { children } = $props();

  async function handleLogout() {
    await logout();
    await goto('/login');
  }

  onMount(() => {
    // Proactively refresh token on initial page load if logged in
    if (authStore.refreshToken) {
      silentRefresh();
    }

    // Phase 4: silently wake parser & reasoner in the background
    warmupServices();

    // Refresh token every 8 minutes (since JWT expires in 15 mins)
    const interval = setInterval(() => {
      if (authStore.refreshToken) {
        silentRefresh();
      }
    }, 8 * 60 * 1000);

    // Phase 5: heartbeat every 10 minutes to keep services warm
    const warmupHeartbeat = setInterval(() => {
      fetch(`${API_BASE}/warmup`).catch(() => {});
    }, 10 * 60 * 1000);

    // Refresh token as soon as user returns to this browser tab
    function handleVisibilityChange() {
      if (document.visibilityState === 'visible' && authStore.refreshToken) {
        silentRefresh();
      }
    }
    document.addEventListener('visibilitychange', handleVisibilityChange);

    return () => {
      clearInterval(interval);
      clearInterval(warmupHeartbeat);
      document.removeEventListener('visibilitychange', handleVisibilityChange);
    };
  });
</script>

<svelte:head>
  <link rel="icon" type="image/png" href={favicon} />
</svelte:head>

<div class="app-root">
  <div class="bg-grid"></div>
  <div class="bg-paper-texture"></div>
  <div class="bg-glow"></div>

  <!-- Minimal session bar -->
  <div class="session-bar">
    <span class="session-email">{authStore.user?.email ?? ''}</span>
    <button class="logout-btn" onclick={handleLogout}>SIGN OUT</button>
  </div>

  <main class="app-main">
    {@render children()}
  </main>

  <!-- Service warm-up status banner (bottom-right, auto-dismisses when ready) -->
  <ServiceWarmupBanner />
</div>

<style>
  .app-root {
    position: relative;
    min-height: 100vh;
    width: 100%;
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: flex-start;
    padding: 1rem 1rem 0.5rem 1rem;
    overflow-x: hidden;
  }

  .app-main {
    position: relative;
    z-index: 1;
    width: 100%;
    max-width: 1680px;
    margin: 0 auto;
    padding: 0 0.5rem;
    display: flex;
    flex-direction: column;
    flex: 1;
  }

  /* Session bar */
  .session-bar {
    position: relative;
    z-index: 10;
    width: 100%;
    max-width: 1680px;
    display: flex;
    align-items: center;
    justify-content: flex-end;
    gap: 1rem;
    padding: 0.4rem 0.5rem 0.75rem;
  }

  .session-email {
    font-family: var(--font-mono);
    font-size: 0.72rem;
    color: var(--text-muted);
    letter-spacing: 0.04em;
  }

  .logout-btn {
    background: none;
    border: 1px solid rgba(0, 240, 255, 0.18);
    border-radius: 4px;
    color: var(--text-muted);
    font-family: var(--font-mono);
    font-size: 0.65rem;
    font-weight: 600;
    letter-spacing: 0.1em;
    padding: 0.3rem 0.7rem;
    cursor: pointer;
    transition: color 0.15s, border-color 0.15s, transform 0.15s;
    box-shadow: 2px 2px 0 rgba(0, 0, 0, 0.2);
  }

  .logout-btn:hover {
    color: var(--pink-accent);
    border-color: var(--pink-accent);
    transform: translateY(-1px);
  }

  /* ─── Light Mode Overrides (Same style as Browse/Upload File button) ─── */
  :global([data-theme="light"]) .logout-btn {
    background: #69818d;
    border: 1px solid #8a9aa8;
    color: #ffffff;
    box-shadow: 2px 2px 0 rgba(0, 0, 0, 0.1);
  }

  :global([data-theme="light"]) .logout-btn:hover {
    background: #5a7380;
    border-color: #69818d;
    color: #ffffff;
  }

  :global([data-theme="light"]) .session-email {
    color: #5a6a78;
  }

  /* ─── Mobile Responsiveness (Edge Spacing & Padding) ─── */
  @media (max-width: 768px) {
    .app-root {
      padding: 0.75rem 1.25rem 1.5rem 1.25rem;
      box-sizing: border-box;
    }

    .app-main {
      padding: 0 0.25rem;
      box-sizing: border-box;
    }

    .session-bar {
      padding: 0.35rem 0.25rem 0.65rem 0.25rem;
      gap: 0.5rem;
      justify-content: space-between;
    }

    .session-email {
      font-size: 0.68rem;
      overflow: hidden;
      text-overflow: ellipsis;
      white-space: nowrap;
      max-width: 180px;
    }

    .logout-btn {
      padding: 0.25rem 0.6rem;
      font-size: 0.62rem;
    }
  }

  @media (max-width: 480px) {
    .app-root {
      padding: 0.5rem 1rem 1.25rem 1rem;
    }

    .app-main {
      padding: 0;
    }
  }
</style>
