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
    <Settings size={15} />
  </button>
</div>

<style>
  .ai-toggle-group {
    display: inline-flex;
    align-items: center;
    gap: 4px;
    height: 34px;
  }

  .ai-toggle-btn {
    display: flex;
    align-items: center;
    gap: 6px;
    height: 34px;
    padding: 0 10px;
    background: rgba(13, 23, 36, 0.95);
    border: 1px solid #132e35;
    border-radius: 6px;
    color: var(--text-muted);
    font-family: var(--font-mono);
    font-size: 0.72rem;
    font-weight: 700;
    cursor: pointer;
    transition: all 0.25s ease;
    box-shadow: 0 2px 2px rgba(0, 0, 0, 0.4);
    user-select: none;
    box-sizing: border-box;
  }

  .ai-toggle-btn:hover {
    border-color: #1e434c;
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
    padding: 2px 5px;
    border-radius: 3px;
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
    height: 34px;
    width: 34px;
    padding: 0;
    background: rgba(13, 23, 36, 0.95);
    border: 1px solid #132e35;
    border-radius: 6px;
    color: var(--text-muted);
    cursor: pointer;
    transition: all 0.25s ease;
    box-shadow: 0 2px 2px rgba(0, 0, 0, 0.4);
    box-sizing: border-box;
  }

  .settings-btn:hover {
    border-color: #1e434c;
    color: var(--text-main);
  }

  /* ─── Light Mode Overrides ─── */
  :global([data-theme="light"]) .ai-toggle-btn {
    background: rgba(175, 179, 183, 0.95);
    border-color: #99a3ab;
    color: #556c80;
    box-shadow: 0 2px 2px rgba(0, 0, 0, 0.4);
  }

  :global([data-theme="light"]) .ai-toggle-btn:hover {
    border-color: #69818d;
    color: #2d3a45;
  }

  :global([data-theme="light"]) .ai-toggle-btn.active {
    background: #69818d;
    border-color: #556c80;
    color: #ffffff;
    box-shadow: 0 2px 2px rgba(0, 0, 0, 0.4);
  }

  :global([data-theme="light"]) .ai-toggle-btn.active .ai-status-badge {
    background: rgba(255, 255, 255, 0.25);
    color: #ffffff;
  }

  :global([data-theme="light"]) .settings-btn {
    background: rgba(175, 179, 183, 0.95);
    border-color: #99a3ab;
    color: #556c80;
    box-shadow: 0 2px 2px rgba(0, 0, 0, 0.4);
  }

  :global([data-theme="light"]) .settings-btn:hover {
    border-color: #69818d;
    color: #ffffff;
    background: #69818d;
  }

  /* ─── Mobile Responsiveness (Smaller Sizing) ─── */
  @media (max-width: 768px) {
    .ai-toggle-group {
      height: 26px;
      gap: 3px;
    }

    .ai-toggle-btn {
      height: 26px;
      padding: 0 6px;
      font-size: 0.62rem;
      gap: 4px;
      border-radius: 4px;
    }

    .ai-status-badge {
      font-size: 0.52rem;
      padding: 1px 4px;
    }

    .settings-btn {
      height: 26px;
      width: 26px;
      border-radius: 4px;
    }
  }
</style>
