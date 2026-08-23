<script lang="ts">
  let {
    id,
    label,
    type = 'text',
    value = $bindable(''),
    placeholder = '',
    error = '',
    disabled = false,
    autocomplete = 'off',
  }: {
    id: string;
    label: string;
    type?: string;
    value?: string;
    placeholder?: string;
    error?: string;
    disabled?: boolean;
    autocomplete?: string;
  } = $props();
</script>

<div class="auth-field">
  <label for={id} class="auth-label">{label}</label>
  <input
    {id}
    {type}
    bind:value
    {placeholder}
    {disabled}
    autocomplete={autocomplete as any}
    class="auth-input"
    class:has-error={!!error}
    aria-invalid={!!error}
    aria-describedby={error ? `${id}-error` : undefined}
  />
  {#if error}
    <p id="{id}-error" class="auth-error" role="alert">{error}</p>
  {/if}
</div>

<style>
  .auth-field {
    display: flex;
    flex-direction: column;
    gap: 0.4rem;
  }

  .auth-label {
    font-family: var(--font-mono);
    font-size: 0.72rem;
    font-weight: 600;
    letter-spacing: 0.1em;
    color: var(--cyan-muted);
    text-transform: uppercase;
  }

  .auth-input {
    width: 100%;
    padding: 0.75rem 1rem;
    background: rgba(0, 0, 0, 0.3);
    border: 1px solid rgba(0, 240, 255, 0.18);
    border-radius: 7px;
    color: var(--text-main);
    font-family: var(--font-body);
    font-size: 0.95rem;
    outline: none;
    transition: border-color 0.2s, box-shadow 0.2s;
  }

  .auth-input::placeholder {
    color: var(--text-dim);
  }

  .auth-input:focus {
    border-color: var(--cyan-primary);
    box-shadow: 0 0 0 3px rgba(0, 240, 255, 0.1);
  }

  .auth-input.has-error {
    border-color: var(--pink-accent);
    box-shadow: 0 0 0 3px rgba(255, 42, 112, 0.1);
  }

  .auth-input:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  .auth-error {
    font-size: 0.8rem;
    color: var(--pink-accent);
    margin: 0;
  }
</style>
