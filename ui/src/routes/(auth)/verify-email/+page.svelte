<script lang="ts">
  import { page } from '$app/stores';
  import { goto } from '$app/navigation';
  import AuthCard from '$lib/components/auth/AuthCard.svelte';
  import AuthButton from '$lib/components/auth/AuthButton.svelte';
  import { verifyEmail } from '$lib/services/authApi';
  import { onMount } from 'svelte';

  type Status = 'verifying' | 'success' | 'error';
  let status = $state<Status>('verifying');
  let errorMessage = $state('');

  onMount(async () => {
    const token = $page.url.searchParams.get('token');
    if (!token) {
      status = 'error';
      errorMessage = 'No verification token found. Please use the link from your email.';
      return;
    }
    try {
      await verifyEmail(token);
      status = 'success';
    } catch (err: any) {
      status = 'error';
      errorMessage = err.message || 'Verification failed. The link may have expired.';
    }
  });
</script>

<svelte:head>
  <title>KAGUA · Verify Email</title>
</svelte:head>

<div class="auth-page">
  <div class="auth-brand">
    <h1 class="brand-name">KAGUA</h1>
    <p class="brand-sub">COMPLIANCE INTELLIGENCE ENGINE</p>
  </div>

  <AuthCard>
    {#if status === 'verifying'}
      <div class="status-state">
        <div class="spinner"></div>
        <h2 class="form-title">Verifying your email…</h2>
        <p class="form-subtitle">Please wait a moment.</p>
      </div>
    {:else if status === 'success'}
      <div class="status-state">
        <div class="status-icon success">✓</div>
        <h2 class="form-title">Email verified!</h2>
        <p class="form-subtitle">
          Your account is now active. You can sign in to access your workspace.
        </p>
        <AuthButton id="verify-goto-login" onclick={() => goto('/login')}>
          SIGN IN NOW
        </AuthButton>
      </div>
    {:else}
      <div class="status-state">
        <div class="status-icon error">✕</div>
        <h2 class="form-title">Verification failed</h2>
        <p class="form-subtitle">{errorMessage}</p>
        <div class="action-row">
          <a href="/register" class="auth-link">Register again</a>
          <span class="separator">·</span>
          <a href="/login" class="auth-link">Sign in</a>
        </div>
      </div>
    {/if}
  </AuthCard>
</div>

<style>
  .auth-page {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 2rem;
    width: 100%;
    max-width: 480px;
    animation: fadeInUp 0.4s ease;
  }

  @keyframes fadeInUp {
    from { opacity: 0; transform: translateY(16px); }
    to   { opacity: 1; transform: translateY(0); }
  }

  .auth-brand {
    text-align: center;
  }

  .brand-name {
    font-family: var(--font-title);
    font-size: 2rem;
    font-weight: 800;
    color: var(--cyan-primary);
    letter-spacing: 0.18em;
    margin: 0;
    text-shadow: 0 0 30px rgba(0, 240, 255, 0.4);
  }

  .brand-sub {
    font-family: var(--font-mono);
    font-size: 0.65rem;
    letter-spacing: 0.2em;
    color: var(--text-muted);
    margin: 4px 0 0;
  }

  .form-title {
    font-family: var(--font-sub);
    font-size: 1.4rem;
    font-weight: 700;
    color: var(--text-main);
    margin: 0 0 0.3rem;
  }

  .form-subtitle {
    font-size: 0.88rem;
    color: var(--text-muted);
    margin: 0 0 1.5rem;
    line-height: 1.5;
  }

  .status-state {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 0.75rem;
    text-align: center;
  }

  .status-icon {
    width: 56px;
    height: 56px;
    border-radius: 50%;
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: 1.5rem;
    font-weight: 700;
  }

  .status-icon.success {
    background: rgba(0, 240, 255, 0.1);
    color: var(--cyan-primary);
    border: 2px solid var(--cyan-primary);
    box-shadow: 0 0 20px rgba(0, 240, 255, 0.25);
  }

  .status-icon.error {
    background: rgba(255, 42, 112, 0.1);
    color: var(--pink-accent);
    border: 2px solid var(--pink-accent);
  }

  .spinner {
    width: 40px;
    height: 40px;
    border: 3px solid rgba(0, 240, 255, 0.15);
    border-top-color: var(--cyan-primary);
    border-radius: 50%;
    animation: spin 0.8s linear infinite;
  }

  @keyframes spin {
    to { transform: rotate(360deg); }
  }

  .action-row {
    display: flex;
    align-items: center;
    gap: 0.75rem;
    font-size: 0.85rem;
    margin-top: 0.5rem;
  }

  .separator {
    color: var(--text-dim);
  }

  .auth-link {
    color: var(--cyan-primary);
    text-decoration: none;
    font-weight: 600;
  }

  .auth-link:hover {
    text-decoration: underline;
  }
</style>
