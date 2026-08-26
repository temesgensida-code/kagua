<script lang="ts">
  import favicon from '$lib/assets/favicon.svg';
  import '../../app.css';

  let { children } = $props();
</script>

<svelte:head>
  <link rel="icon" href={favicon} />
</svelte:head>

<div class="app-root">
  <div class="bg-grid"></div>
  <div class="bg-paper-texture"></div>
  <div class="bg-glow"></div>
  <main class="auth-center">
    {@render children()}
  </main>
</div>

<style>
  .auth-center {
    position: relative;
    z-index: 1;
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    min-height: 100vh;
    padding: 2rem 1rem;
    width: 100%;
  }

  @media (max-width: 768px) {
    .auth-center {
      padding: 1rem 0.75rem;
    }
  }
</style>
