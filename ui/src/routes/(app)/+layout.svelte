<script lang="ts">
  import favicon from '$lib/assets/favicon.png';
  import '../../app.css';
  import { authStore } from '$lib/services/authStore.svelte';
  import { logout } from '$lib/services/authApi';
  import { goto } from '$app/navigation';

  let { children } = $props();

  async function handleLogout() {
    await logout();
    await goto('/login');
  }
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

  /* ─── Mobile Responsiveness ─── */
  @media (max-width: 640px) {
    .app-root {
      padding: 0.5rem 0.5rem 1rem 0.5rem;
    }

    .app-main {
      padding: 0;
    }

    .session-bar {
      padding: 0.25rem 0.25rem 0.5rem;
      gap: 0.5rem;
      justify-content: space-between;
    }

    .session-email {
      font-size: 0.68rem;
      overflow: hidden;
      text-overflow: ellipsis;
      white-space: nowrap;
      max-width: 200px;
    }

    .logout-btn {
      padding: 0.25rem 0.55rem;
      font-size: 0.6rem;
    }
  }
</style>
