<script lang="ts">
  import AuthCard from '$lib/components/auth/AuthCard.svelte';
  import AuthInput from '$lib/components/auth/AuthInput.svelte';
  import AuthButton from '$lib/components/auth/AuthButton.svelte';
  import { forgotPassword } from '$lib/services/authApi';

  let email = $state('');
  let loading = $state(false);
  let error = $state('');
  let sent = $state(false);

  async function handleSubmit(e: Event) {
    e.preventDefault();
    error = '';

    if (!email.includes('@')) {
      error = 'Please enter a valid email address.';
      return;
    }

    loading = true;
    try {
      await forgotPassword(email.trim().toLowerCase());
      sent = true;
    } catch (err: any) {
      error = err.message || 'Something went wrong. Please try again.';
    } finally {
      loading = false;
    }
  }
</script>

<svelte:head>
  <title>KAGUA · Forgot Password</title>
  <meta name="description" content="Reset your Kagua account password." />
</svelte:head>

<div class="auth-page">
  <div class="auth-brand">
    <h1 class="brand-name">KAGUA</h1>
    <p class="brand-sub">COMPLIANCE INTELLIGENCE ENGINE</p>
  </div>

  <AuthCard>
    {#if sent}
      <div class="sent-state">
        <div class="sent-icon">✉</div>
        <h2 class="form-title">Check your inbox</h2>
        <p class="form-subtitle">
          If an account with <strong class="email-hl">{email}</strong> exists, we've sent a password reset link. The link expires in <strong>1 hour</strong>.
        </p>
        <a href="/login" class="auth-link">← Back to sign in</a>
      </div>
    {:else}
      <div class="back-row">
        <a href="/login" class="back-link">← Back to sign in</a>
      </div>

      <h2 class="form-title">Forgot password?</h2>
      <p class="form-subtitle">
        Enter your account email and we'll send you a reset link.
      </p>

      <form onsubmit={handleSubmit} class="auth-form" novalidate>
        <AuthInput
          id="forgot-email"
          label="Email address"
          type="email"
          bind:value={email}
          placeholder="you@example.com"
          autocomplete="email"
          disabled={loading}
        />

        {#if error}
          <div class="form-error" role="alert">{error}</div>
        {/if}

        <AuthButton id="forgot-submit" type="submit" {loading}>
          SEND RESET LINK
        </AuthButton>
      </form>
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

  .auth-brand { text-align: center; }

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

  .back-row {
    margin-bottom: 1.25rem;
  }

  .back-link {
    font-size: 0.82rem;
    color: var(--text-muted);
    text-decoration: none;
    transition: color 0.15s;
  }

  .back-link:hover { color: var(--cyan-primary); }

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
    margin: 0 0 1.75rem;
    line-height: 1.5;
  }

  .auth-form {
    display: flex;
    flex-direction: column;
    gap: 1.1rem;
  }

  .form-error {
    padding: 0.7rem 1rem;
    background: rgba(255, 42, 112, 0.08);
    border: 1px solid rgba(255, 42, 112, 0.3);
    border-radius: 6px;
    color: var(--pink-accent);
    font-size: 0.85rem;
  }

  /* Sent state */
  .sent-state {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 0.75rem;
    text-align: center;
  }

  .sent-icon {
    font-size: 2.5rem;
    filter: drop-shadow(0 0 12px rgba(0,240,255,0.5));
  }

  .email-hl {
    color: var(--cyan-primary);
    font-weight: 600;
  }

  .auth-link {
    color: var(--cyan-primary);
    text-decoration: none;
    font-weight: 600;
    font-size: 0.875rem;
    margin-top: 0.5rem;
  }

  .auth-link:hover { text-decoration: underline; }
</style>
