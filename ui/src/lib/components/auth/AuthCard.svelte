<script lang="ts">
  let { children } = $props();
</script>

<div class="auth-card">
  <div class="auth-card-glow"></div>
  <div class="auth-card-inner">
    {@render children()}
  </div>
</div>

<style>
  .auth-card {
    position: relative;
    width: 100%;
    max-width: 440px;
    margin: 0 auto;
  }

  .auth-card-glow {
    position: absolute;
    inset: -1px;
    border-radius: 14px;
    background: linear-gradient(135deg, rgba(0,240,255,0.25), rgba(255,42,112,0.08));
    z-index: 0;
    pointer-events: none;
  }

  .auth-card-inner {
    position: relative;
    z-index: 1;
    background: var(--bg-panel);
    border: 1px solid rgba(0, 240, 255, 0.14);
    border-radius: 14px;
    padding: 2.5rem 2rem;
    backdrop-filter: blur(20px);
  }
</style>
