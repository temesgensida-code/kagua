<script lang="ts">
  import { aiSettings } from '$lib/services/aiSettings.svelte';
  import { soundState } from '$lib/services/sound.svelte';
  import { Sparkles, Settings } from '@lucide/svelte';

  interface Props {
    onOpenSettings: () => void;
  }

  let { onOpenSettings }: Props = $props();

  function toggleAI() {
    soundState.playClick();
    const nextState = !aiSettings.enabled;
    aiSettings.setEnabled(nextState);
    if (nextState && !aiSettings.apiKey) {
      onOpenSettings();
    }
  }
</script>

<div class="ai-toggle-group">
  <button
    type="button"
    class="ai-toggle-btn"
    class:active={aiSettings.enabled}
    onclick={toggleAI}
    title={aiSettings.enabled ? `AI Mode Active (${aiSettings.selectedModel.name})` : 'AI Mode Disabled (In-Memory Prolog Only)'}
  >
    <Sparkles size={14} class="ai-icon" />
    <span class="ai-label">AI</span>
    <span class="ai-status-badge">{aiSettings.enabled ? 'ON' : 'OFF'}</span>
  </button>

  <button
    type="button"
    class="settings-btn"
    onclick={() => {
      soundState.playClick();
      onOpenSettings();
    }}
    title="AI & Compliance Settings"
  >
    <Settings size={14} />
  </button>
</div>

<style>
  .ai-toggle-group {
    display: inline-flex;
    align-items: center;
    gap: 4px;
  }

  .ai-toggle-btn {
    display: flex;
    align-items: center;
    gap: 6px;
    padding: 4px 10px;
    background: rgba(13, 23, 36, 0.8);
    border: 1px solid rgba(255, 255, 255, 0.12);
    border-radius: 4px;
    color: var(--text-muted);
    font-family: var(--font-mono);
    font-size: 0.72rem;
    font-weight: 700;
    cursor: pointer;
    transition: all 0.2s cubic-bezier(0.16, 1, 0.3, 1);
    user-select: none;
  }

  .ai-toggle-btn:hover {
    border-color: rgba(0, 240, 255, 0.4);
    color: var(--text-main);
  }

  .ai-toggle-btn.active {
    background: rgba(0, 255, 136, 0.08);
    border-color: #00ff88;
    color: #00ff88;
    box-shadow: 0 0 10px rgba(0, 255, 136, 0.25);
  }

  .ai-toggle-btn :global(.ai-icon) {
    transition: transform 0.3s ease;
  }

  .ai-toggle-btn.active :global(.ai-icon) {
    transform: rotate(15deg) scale(1.1);
  }

  .ai-label {
    letter-spacing: 1px;
  }

  .ai-status-badge {
    font-size: 0.6rem;
    font-weight: 800;
    padding: 1px 5px;
    border-radius: 2px;
    background: rgba(255, 255, 255, 0.1);
    color: var(--text-muted);
  }

  .ai-toggle-btn.active .ai-status-badge {
    background: rgba(0, 255, 136, 0.2);
    color: #00ff88;
  }

  .settings-btn {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    padding: 6px;
    background: rgba(13, 23, 36, 0.8);
    border: 1px solid rgba(255, 255, 255, 0.12);
    border-radius: 4px;
    color: var(--text-muted);
    cursor: pointer;
    transition: all 0.2s cubic-bezier(0.16, 1, 0.3, 1);
  }

  .settings-btn:hover {
    border-color: var(--cyan-primary);
    color: var(--cyan-primary);
    transform: rotate(30deg);
  }

  /* Light Theme Overrides */
  :global([data-theme="light"]) .ai-toggle-btn {
    background: #c2c6ca;
    border-color: #a0a6ac;
    color: #5a6a78;
  }

  :global([data-theme="light"]) .ai-toggle-btn.active {
    background: #69818d;
    border-color: #4a5a68;
    color: #ffffff;
    box-shadow: none;
  }

  :global([data-theme="light"]) .ai-toggle-btn.active .ai-status-badge {
    background: rgba(255, 255, 255, 0.25);
    color: #ffffff;
  }

  :global([data-theme="light"]) .settings-btn {
    background: #c2c6ca;
    border-color: #a0a6ac;
    color: #5a6a78;
  }
</style>
