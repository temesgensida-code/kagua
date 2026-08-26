<script lang="ts">
  import { goto } from '$app/navigation';
  import AuthCard from '$lib/components/auth/AuthCard.svelte';
  import AuthInput from '$lib/components/auth/AuthInput.svelte';
  import AuthButton from '$lib/components/auth/AuthButton.svelte';
  import PasswordStrength from '$lib/components/auth/PasswordStrength.svelte';
  import { register } from '$lib/services/authApi';

  let email = $state('');
  let password = $state('');
  let confirmPassword = $state('');
  let loading = $state(false);
  let error = $state('');
  let success = $state(false);

  let emailError = $state('');
  let passwordError = $state('');
  let confirmError = $state('');

  function validate() {
    emailError = '';
    passwordError = '';
    confirmError = '';
    let ok = true;

    if (!email.includes('@') || !email.includes('.')) {
      emailError = 'Enter a valid email address.';
      ok = false;
    }
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
    if (!validate()) return;

    loading = true;
    try {
      await register(email.trim().toLowerCase(), password);
      success = true;
    } catch (err: any) {
      error = err.message || 'Registration failed. Please try again.';
    } finally {
      loading = false;
    }
  }
</script>

<svelte:head>
  <title>KAGUA · Create Account</title>
  <meta name="description" content="Create your Kagua compliance intelligence account." />
</svelte:head>

<div class="auth-page">
  <!-- Brand header -->
  <div class="auth-brand">
    <h1 class="brand-name">KAGUA</h1>
    <p class="brand-sub">COMPLIANCE INTELLIGENCE ENGINE</p>
  </div>

  <AuthCard>
    {#if success}
      <div class="success-state">
        <div class="success-icon">✉</div>
        <h2 class="form-title">Check your inbox</h2>
        <p class="form-subtitle">
          We sent a verification link to <strong class="email-highlight">{email}</strong>.
          Click the link to activate your account.
        </p>
        <p class="resend-hint">
          Didn't receive it?
          <a href="/login" class="auth-link">Go to sign in</a>
          to request a new link.
        </p>
      </div>
    {:else}
      <h2 class="form-title">Create account</h2>
      <p class="form-subtitle">Join Kagua to analyse contracts for labour law compliance.</p>

      <form onsubmit={handleSubmit} class="auth-form" novalidate>
        <AuthInput
          id="register-email"
          label="Email address"
          type="email"
          bind:value={email}
          placeholder="you@example.com"
          autocomplete="email"
          error={emailError}
          disabled={loading}
        />
        <div class="field-stack">
          <AuthInput
            id="register-password"
            label="Password"
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
          id="register-confirm"
          label="Confirm password"
          type="password"
          bind:value={confirmPassword}
          placeholder="Re-enter password"
          autocomplete="new-password"
          error={confirmError}
          disabled={loading}
        />

        {#if error}
          <div class="form-error" role="alert">{error}</div>
        {/if}

        <AuthButton id="register-submit" type="submit" {loading}>
          CREATE ACCOUNT
        </AuthButton>
      </form>

      <p class="auth-footer">
        Already have an account?
        <a href="/login" class="auth-link">Sign in</a>
      </p>
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
    letter-spacing: 0.02em;
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
    line-height: 1.4;
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

  /* Success state */
  .success-state {
    text-align: center;
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 0.75rem;
  }

  .success-icon {
    font-size: 2.5rem;
    filter: drop-shadow(0 0 12px rgba(0,240,255,0.5));
  }

  .email-highlight {
    color: var(--cyan-primary);
    font-weight: 600;
  }

  .resend-hint {
    font-size: 0.82rem;
    color: var(--text-muted);
    margin-top: 0.5rem;
  }

  @media (max-width: 768px) {
    .auth-page {
      gap: 1.25rem;
      max-width: 100%;
    }

    .brand-name {
      font-size: 1.4rem;
      letter-spacing: 0.12em;
    }

    .brand-sub {
      font-size: 0.55rem;
      letter-spacing: 0.14em;
    }

    .form-title {
      font-size: 1.15rem;
      margin-bottom: 0.2rem;
    }

    .form-subtitle {
      font-size: 0.76rem;
      margin-bottom: 1.1rem;
    }

    .auth-form {
      gap: 0.75rem;
    }

    .field-stack {
      gap: 0.2rem;
    }

    .auth-footer {
      margin-top: 1rem;
      font-size: 0.78rem;
    }

    .success-icon {
      font-size: 2rem;
    }
  }
</style>
