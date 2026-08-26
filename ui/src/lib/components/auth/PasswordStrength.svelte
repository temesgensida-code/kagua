<script lang="ts">
  let { password }: { password: string } = $props();

  const score = $derived.by(() => {
    let s = 0;
    if (password.length >= 8) s++;
    if (password.length >= 12) s++;
    if (/[A-Z]/.test(password)) s++;
    if (/[0-9]/.test(password)) s++;
    if (/[^A-Za-z0-9]/.test(password)) s++;
    return s;
  });

  const label = $derived(
    score === 0 ? '' :
    score <= 1 ? 'Weak' :
    score <= 2 ? 'Fair' :
    score <= 3 ? 'Good' :
    score <= 4 ? 'Strong' : 'Very Strong'
  );

  const color = $derived(
    score === 0 ? 'transparent' :
    score <= 1 ? '#ff2a70' :
    score <= 2 ? '#f59e0b' :
    score <= 3 ? '#6ee7b7' :
    '#00f0ff'
  );
</script>

{#if password.length > 0}
  <div class="strength-wrap">
    <div class="bars">
      {#each Array(5) as _, i}
        <div
          class="bar"
          style:background={i < score ? color : 'rgba(255,255,255,0.06)'}
          style:transition="background 0.3s ease {i * 0.05}s"
        ></div>
      {/each}
    </div>
    <span class="strength-label" style:color>{label}</span>
  </div>
{/if}

<style>
  .strength-wrap {
    display: flex;
    align-items: center;
    gap: 0.75rem;
    margin-top: 0.25rem;
  }

  .bars {
    display: flex;
    gap: 4px;
    flex: 1;
  }

  .bar {
    height: 4px;
    flex: 1;
    border-radius: 2px;
  }

  .strength-label {
    font-family: var(--font-mono);
    font-size: 0.7rem;
    font-weight: 600;
    letter-spacing: 0.06em;
    min-width: 60px;
    text-align: right;
  }

  @media (max-width: 768px) {
    .strength-wrap {
      gap: 0.5rem;
    }

    .bar {
      height: 3px;
    }

    .strength-label {
      font-size: 0.62rem;
      min-width: 50px;
    }
  }
</style>
