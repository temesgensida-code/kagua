<script lang="ts">
  import { goto } from '$app/navigation';
  import AuthCard from '$lib/components/auth/AuthCard.svelte';
  import AuthInput from '$lib/components/auth/AuthInput.svelte';
  import AuthButton from '$lib/components/auth/AuthButton.svelte';
  import { login, resendVerification } from '$lib/services/authApi';

  let email = $state('');
  let password = $state('');
  let rememberMe = $state(false);
  let loading = $state(false);
  let error = $state('');
  let showResend = $state(false);
  let resendLoading = $state(false);
  let resendDone = $state(false);

  async function handleSubmit(e: Event) {
    e.preventDefault();
    error = '';
    showResend = false;

    if (!email || !password) {
      error = 'Please fill in all fields.';
      return;
    }

    loading = true;
    try {
      await login(email.trim().toLowerCase(), password, rememberMe);
      await goto('/');
    } catch (err: any) {
      error = err.message || 'Sign in failed. Please try again.';
      if (err.message?.includes('email_not_verified') || err.message?.includes('verify your email')) {
        showResend = true;
      }
    } finally {
      loading = false;
    }
  }

  async function handleResend() {
    resendLoading = true;
    try {
      await resendVerification(email.trim().toLowerCase());
      resendDone = true;
    } catch {
      // ignore
    } finally {
      resendLoading = false;
    }
  }
</script>

<svelte:head>
  <title>KAGUA · Sign In</title>
  <meta name="description" content="Sign in to your Kagua compliance intelligence account." />
</svelte:head>

<div class="auth-page">
  <div class="auth-brand">
    <h1 class="brand-name">KAGUA</h1>
    <p class="brand-sub">COMPLIANCE INTELLIGENCE ENGINE</p>
  </div>

  <AuthCard>
    <h2 class="form-title">Sign in</h2>
    <p class="form-subtitle">Access your compliance analysis workspace.</p>

    <form onsubmit={handleSubmit} class="auth-form" novalidate>
      <AuthInput
        id="login-email"
        label="Email address"
        type="email"
        bind:value={email}
        placeholder="you@example.com"
        autocomplete="email"
        disabled={loading}
      />
      <AuthInput
        id="login-password"
        label="Password"
        type="password"
        bind:value={password}
        placeholder="Your password"
        autocomplete="current-password"
        disabled={loading}
      />

      <div class="login-options">
        <label class="remember-me" for="remember-me-check">
          <input
            id="remember-me-check"
            type="checkbox"
            bind:checked={rememberMe}
            class="remember-checkbox"
          />
          <span class="remember-label">Remember me</span>
        </label>
        <a href="/forgot-password" class="forgot-link">Forgot password?</a>
      </div>

      {#if error}
        <div class="form-error" role="alert">
          <span>{error}</span>
          {#if showResend && !resendDone}
            <button
              type="button"
              class="resend-btn"
              disabled={resendLoading}
              onclick={handleResend}
            >
              {resendLoading ? 'Sending…' : 'Resend verification email'}
            </button>
          {/if}
          {#if resendDone}
            <span class="resend-ok"> ✓ Verification email sent.</span>
          {/if}
        </div>
      {/if}

      <AuthButton id="login-submit" type="submit" {loading}>
        SIGN IN
      </AuthButton>
    </form>

    <p class="auth-footer">
      Don't have an account?
      <a href="/register" class="auth-link">Create one</a>
    </p>
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
    letter-spacing: 0.02em;
  }

  .form-subtitle {
    font-size: 0.88rem;
    color: var(--text-muted);
    margin: 0 0 1.75rem;
  }

  .auth-form {
    display: flex;
    flex-direction: column;
    gap: 1.1rem;
  }

  .login-options {
    display: flex;
    align-items: center;
    justify-content: space-between;
    font-size: 0.82rem;
  }

  .remember-me {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    cursor: pointer;
  }

  .remember-checkbox {
    width: 14px;
    height: 14px;
    accent-color: var(--cyan-primary);
    cursor: pointer;
  }

  .remember-label {
    color: var(--text-muted);
    user-select: none;
  }

  .forgot-link {
    color: var(--cyan-muted);
    text-decoration: none;
    transition: color 0.15s;
  }

  .forgot-link:hover {
    color: var(--cyan-primary);
  }

  .form-error {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
    padding: 0.75rem 1rem;
    background: rgba(255, 42, 112, 0.08);
    border: 1px solid rgba(255, 42, 112, 0.3);
    border-radius: 6px;
    color: var(--pink-accent);
    font-size: 0.85rem;
    line-height: 1.4;
  }

  .resend-btn {
    background: none;
    border: none;
    color: var(--cyan-primary);
    font-size: 0.82rem;
    font-weight: 600;
    cursor: pointer;
    padding: 0;
    text-align: left;
    text-decoration: underline;
  }

  .resend-btn:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  .resend-ok {
    color: #6ee7b7;
    font-size: 0.8rem;
  }

  .auth-footer {
    margin-top: 1.5rem;
    text-align: center;
    font-size: 0.875rem;
    color: var(--text-muted);
  }

  .auth-link {
    color: var(--cyan-primary);
    text-decoration: none;
    font-weight: 600;
    transition: opacity 0.15s;
  }

  .auth-link:hover {
    opacity: 0.8;
    text-decoration: underline;
  }
</style>
