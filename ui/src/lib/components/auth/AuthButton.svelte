<script lang="ts">
  let {
    id,
    type = 'button',
    disabled = false,
    loading = false,
    variant = 'primary',
    onclick,
    children,
  }: {
    id: string;
    type?: 'button' | 'submit';
    disabled?: boolean;
    loading?: boolean;
    variant?: 'primary' | 'secondary';
    onclick?: () => void;
    children: any;
  } = $props();
</script>

<button
  {id}
  {type}
  disabled={disabled || loading}
  class="auth-btn auth-btn--{variant}"
  {onclick}
>
  {#if loading}
    <span class="btn-spinner" aria-hidden="true"></span>
    <span>PROCESSING...</span>
  {:else}
    {@render children()}
  {/if}
</button>

<style>
  .auth-btn {
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 0.5rem;
    width: 100%;
    padding: 0.85rem 1.5rem;
    border: none;
    border-radius: 7px;
    font-family: var(--font-mono);
    font-size: 0.82rem;
    font-weight: 700;
    letter-spacing: 0.1em;
    text-transform: uppercase;
    cursor: pointer;
    transition: opacity 0.2s, transform 0.15s, box-shadow 0.2s;
  }

  .auth-btn:disabled {
    opacity: 0.6;
    cursor: not-allowed;
    transform: none !important;
  }

  .auth-btn:not(:disabled):hover {
    transform: translateY(-1px);
  }

  .auth-btn:not(:disabled):active {
    transform: translateY(0);
  }

  .auth-btn--primary {
    background: linear-gradient(135deg, #00f0ff, #0090a8);
    color: #070b10;
    box-shadow: 0 4px 20px rgba(0, 240, 255, 0.25);
  }

  .auth-btn--primary:not(:disabled):hover {
    box-shadow: 0 6px 30px rgba(0, 240, 255, 0.4);
  }

  .auth-btn--secondary {
    background: transparent;
    border: 1px solid rgba(0, 240, 255, 0.25);
    color: var(--cyan-muted);
  }

  .auth-btn--secondary:not(:disabled):hover {
    border-color: var(--cyan-primary);
    color: var(--cyan-primary);
    box-shadow: 0 0 12px rgba(0, 240, 255, 0.15);
  }

  /* ─── Light Mode Overrides (Same style as Browse/Upload File button) ─── */
  :global([data-theme="light"]) .auth-btn--primary {
    background: #69818d;
    border: 1px solid #8a9aa8;
    color: #ffffff;
    box-shadow: 0 4px 12px rgba(105, 129, 141, 0.25);
  }

  :global([data-theme="light"]) .auth-btn--primary:not(:disabled):hover {
    background: #5a7380;
    border-color: #69818d;
    box-shadow: 0 6px 16px rgba(90, 115, 128, 0.35);
  }

  :global([data-theme="light"]) .auth-btn--secondary {
    background: #c2c6ca;
    border: 1px solid #8a9aa8;
    color: #2d3a45;
  }

  :global([data-theme="light"]) .auth-btn--secondary:not(:disabled):hover {
    background: #afb3b7;
    border-color: #69818d;
    color: #0f172a;
  }

  :global([data-theme="light"]) .btn-spinner {
    border: 2px solid rgba(255, 255, 255, 0.4);
    border-top-color: #ffffff;
  }

  .btn-spinner {
    width: 14px;
    height: 14px;
    border: 2px solid rgba(7, 11, 16, 0.4);
    border-top-color: #070b10;
    border-radius: 50%;
    animation: spin 0.6s linear infinite;
    flex-shrink: 0;
  }

  @keyframes spin {
    to { transform: rotate(360deg); }
  }

  @media (max-width: 768px) {
    .auth-btn {
      padding: 0.6rem 1rem;
      font-size: 0.75rem;
      letter-spacing: 0.08em;
      border-radius: 6px;
    }

    .btn-spinner {
      width: 12px;
      height: 12px;
    }
  }
</style>
