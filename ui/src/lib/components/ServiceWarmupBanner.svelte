<script lang="ts">
  import { warmupState, type WarmupStatus, type ServiceStatus } from '$lib/services/serviceWarmup';

  // Auto-dismiss the banner 3 seconds after all services are ready
  let dismissed = $state(false);
  let dismissTimer: ReturnType<typeof setTimeout> | null = null;

  $effect(() => {
    if (warmupState.status === 'ready') {
      dismissTimer = setTimeout(() => {
        dismissed = true;
      }, 3000);
    }
    return () => {
      if (dismissTimer) clearTimeout(dismissTimer);
    };
  });

  const icons: Record<WarmupStatus, string> = {
    idle:    '',
    warming: '⚡',
    ready:   '✓',
    failed:  '⚠',
  };

  const labels: Record<WarmupStatus, string> = {
    idle:    '',
    warming: 'Waking analysis services…',
    ready:   'All services ready',
    failed:  'Services slow to start — upload may still work',
  };

  function svcLabel(s: ServiceStatus): string {
    return s === 'online' ? '✓' : '…';
  }
</script>

{#if warmupState.status !== 'idle' && !dismissed}
  <div
    class="warmup-banner"
    class:state-warming={warmupState.status === 'warming'}
    class:state-ready={warmupState.status === 'ready'}
    class:state-failed={warmupState.status === 'failed'}
    role="status"
    aria-live="polite"
  >
    <span class="warmup-icon">{icons[warmupState.status]}</span>
    <span class="warmup-label">{labels[warmupState.status]}</span>

    {#if warmupState.status === 'warming'}
      <span class="warmup-services">
        <span class="svc" class:online={warmupState.parser === 'online'}>
          Parser&nbsp;{svcLabel(warmupState.parser)}
        </span>
        <span class="svc" class:online={warmupState.reasoner === 'online'}>
          Reasoner&nbsp;{svcLabel(warmupState.reasoner)}
        </span>
      </span>
    {/if}
  </div>
{/if}

<style>
  .warmup-banner {
    position: fixed;
    bottom: 1.5rem;
    right: 1.5rem;
    z-index: 9100;
    display: flex;
    align-items: center;
    gap: 0.55rem;

    /* glassmorphism card */
    background: rgba(8, 12, 20, 0.85);
    border: 1px solid rgba(0, 240, 255, 0.18);
    backdrop-filter: blur(14px);
    -webkit-backdrop-filter: blur(14px);
    border-radius: 10px;
    padding: 0.55rem 1.1rem;

    font-family: var(--font-mono, 'JetBrains Mono', monospace);
    font-size: 0.72rem;
    color: rgba(200, 220, 255, 0.7);
    box-shadow: 0 6px 32px rgba(0, 0, 0, 0.45), 0 0 0 1px rgba(0, 240, 255, 0.05);

    animation: warmup-slide-in 0.35s cubic-bezier(0.22, 1, 0.36, 1) both;
  }

  /* ── state colours ─────────────────────────────────────────── */
  .state-warming {
    border-color: rgba(0, 200, 255, 0.22);
  }

  .state-ready {
    border-color: rgba(0, 255, 160, 0.4);
    color: #00ffa0;
    background: rgba(0, 30, 18, 0.88);
  }

  .state-failed {
    border-color: rgba(255, 170, 0, 0.38);
    color: #ffaa00;
    background: rgba(30, 18, 0, 0.88);
  }

  /* ── icon ───────────────────────────────────────────────────── */
  .warmup-icon {
    font-size: 0.9rem;
    line-height: 1;
    flex-shrink: 0;
  }

  /* ── per-service pills ──────────────────────────────────────── */
  .warmup-services {
    display: flex;
    gap: 0.65rem;
    margin-left: 0.45rem;
    padding-left: 0.65rem;
    border-left: 1px solid rgba(255, 255, 255, 0.08);
  }

  .svc {
    color: rgba(160, 180, 210, 0.55);
    transition: color 0.4s ease;
    letter-spacing: 0.03em;
  }

  .svc.online {
    color: #00ffa0;
  }

  /* ── light-mode overrides ───────────────────────────────────── */
  :global([data-theme='light']) .warmup-banner {
    background: rgba(245, 248, 255, 0.92);
    border-color: rgba(0, 140, 200, 0.25);
    color: #4a6070;
    box-shadow: 0 4px 20px rgba(0, 0, 0, 0.12);
  }

  :global([data-theme='light']) .state-ready {
    background: rgba(230, 255, 245, 0.95);
    border-color: rgba(0, 180, 100, 0.4);
    color: #009960;
  }

  :global([data-theme='light']) .state-failed {
    background: rgba(255, 248, 230, 0.95);
    border-color: rgba(200, 130, 0, 0.4);
    color: #b07000;
  }

  :global([data-theme='light']) .svc.online {
    color: #009960;
  }

  /* ── mobile ─────────────────────────────────────────────────── */
  @media (max-width: 480px) {
    .warmup-banner {
      bottom: 1rem;
      right: 0.75rem;
      left: 0.75rem;
      justify-content: center;
      font-size: 0.69rem;
    }
  }

  /* ── animation ──────────────────────────────────────────────── */
  @keyframes warmup-slide-in {
    from {
      opacity: 0;
      transform: translateY(14px) scale(0.97);
    }
    to {
      opacity: 1;
      transform: translateY(0) scale(1);
    }
  }
</style>
