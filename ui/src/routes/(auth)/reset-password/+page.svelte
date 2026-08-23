<script lang="ts">
  import { page } from '$app/stores';
  import { goto } from '$app/navigation';
  import AuthCard from '$lib/components/auth/AuthCard.svelte';
  import AuthInput from '$lib/components/auth/AuthInput.svelte';
  import AuthButton from '$lib/components/auth/AuthButton.svelte';
  import PasswordStrength from '$lib/components/auth/PasswordStrength.svelte';
  import { resetPassword } from '$lib/services/authApi';

  let password = $state('');
  let confirmPassword = $state('');
  let loading = $state(false);
  let error = $state('');
  let passwordError = $state('');
  let confirmError = $state('');
  let success = $state(false);

  const token = $derived($page.url.searchParams.get('token') ?? '');

  function validate() {
    passwordError = '';
    confirmError = '';
    let ok = true;
    if (password.length < 8) {
      passwordError = 'Password must be at least 8 characters.';
      ok = false;
    }
    if (password !== confirmPassword) {
      confirmError = 'Passwords do not match.';
      ok = false;
    }
    return ok;
  }

  async function handleSubmit(e: Event) {
    e.preventDefault();
    error = '';
    if (!token) {
      error = 'Missing reset token. Please use the link from your email.';
      return;
    }
    if (!validate()) return;

    loading = true;
    try {
      await resetPassword(token, password);
      success = true;
    } catch (err: any) {
      error = err.message || 'Password reset failed. The link may have expired.';
    } finally {
      loading = false;
    }
  }
</script>

<svelte:head>
  <title>KAGUA · Reset Password</title>
</svelte:head>

<div class="auth-page">
  <div class="auth-brand">
    <h1 class="brand-name">KAGUA</h1>
    <p class="brand-sub">COMPLIANCE INTELLIGENCE ENGINE</p>
  </div>

  <AuthCard>
    {#if success}
      <div class="success-state">
        <div class="success-icon">✓</div>
        <h2 class="form-title">Password reset!</h2>
        <p class="form-subtitle">
          Your password has been updated. You can now sign in with your new credentials.
        </p>
        <AuthButton id="reset-goto-login" onclick={() => goto('/login')}>
          SIGN IN NOW
        </AuthButton>
      </div>
    {:else}
      <h2 class="form-title">Set new password</h2>
      <p class="form-subtitle">Choose a strong password for your Kagua account.</p>

      <form onsubmit={handleSubmit} class="auth-form" novalidate>
        <div class="field-stack">
          <AuthInput
            id="reset-password"
            label="New password"
            type="password"
            bind:value={password}
            placeholder="Min. 8 characters"
            autocomplete="new-password"
            error={passwordError}
            disabled={loading}
          />
          <PasswordStrength {password} />
        </div>

        <AuthInput
          id="reset-confirm"
          label="Confirm new password"
          type="password"
          bind:value={confirmPassword}
          placeholder="Re-enter new password"
          autocomplete="new-password"
          error={confirmError}
          disabled={loading}
        />

        {#if error}
          <div class="form-error" role="alert">{error}</div>
        {/if}

        <AuthButton id="reset-submit" type="submit" {loading}>
          RESET PASSWORD
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

  .field-stack {
    display: flex;
    flex-direction: column;
    gap: 0.3rem;
  }

  .form-error {
    padding: 0.7rem 1rem;
    background: rgba(255, 42, 112, 0.08);
    border: 1px solid rgba(255, 42, 112, 0.3);
    border-radius: 6px;
    color: var(--pink-accent);
    font-size: 0.85rem;
  }

  .success-state {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 0.75rem;
    text-align: center;
  }

  .success-icon {
    width: 56px;
    height: 56px;
    border-radius: 50%;
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: 1.5rem;
    font-weight: 700;
    background: rgba(0, 240, 255, 0.1);
    color: var(--cyan-primary);
    border: 2px solid var(--cyan-primary);
    box-shadow: 0 0 20px rgba(0, 240, 255, 0.25);
  }
</style>
