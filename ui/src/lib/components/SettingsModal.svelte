<script lang="ts">
  import { aiSettings, AI_MODELS, type AIModelOption } from '$lib/services/aiSettings.svelte';
  import { soundState } from '$lib/services/sound.svelte';
  import { X, Eye, EyeOff, Key, Cpu, Sparkles, CheckCircle2, AlertCircle } from '@lucide/svelte';

  interface Props {
    isOpen: boolean;
    onClose: () => void;
  }

  let { isOpen, onClose }: Props = $props();

  let tempApiKey = $state(aiSettings.apiKey);
  let tempModelId = $state(aiSettings.selectedModelId);
  let showKey = $state(false);

  $effect(() => {
    if (isOpen) {
      tempApiKey = aiSettings.apiKey;
      tempModelId = aiSettings.selectedModelId;
    }
  });

  function handleSave() {
    soundState.playClick();
    aiSettings.setApiKey(tempApiKey);
    aiSettings.setSelectedModelId(tempModelId);
    onClose();
  }

  function handleBackdropClick(e: MouseEvent) {
    if (e.target === e.currentTarget) {
      soundState.playClick();
      onClose();
    }
  }

  function handleKeyDown(e: KeyboardEvent) {
    if (e.key === 'Escape' && isOpen) {
      soundState.playClick();
      onClose();
    }
  }

  // Group models by provider
  const googleModels = AI_MODELS.filter(m => m.provider === 'google');
  const xaiModels = AI_MODELS.filter(m => m.provider === 'xai');
  const openaiModels = AI_MODELS.filter(m => m.provider === 'openai');
</script>

<svelte:window onkeydown={handleKeyDown} />

{#if isOpen}
  <div
    class="modal-backdrop"
    onclick={handleBackdropClick}
    onkeydown={handleKeyDown}
    role="dialog"
    aria-modal="true"
    tabindex="-1"
  >
    <div class="modal-card">
      <div class="modal-header">
        <div class="modal-title-row">
          <Sparkles size={16} class="title-icon" />
          <h3 class="modal-title">AI ENGINE CONFIGURATION</h3>
        </div>
        <button
          type="button"
          class="btn-close"
          onclick={() => {
            soundState.playClick();
            onClose();
          }}
        >
          <X size={16} />
        </button>
      </div>

      <div class="modal-body">
        <!-- API Key Setter -->
        <div class="form-group">
          <label for="ai-api-key" class="form-label">
            <Key size={13} />
            <span>AI PROVIDER API KEY</span>
          </label>
          <div class="input-wrapper">
            <input
              id="ai-api-key"
              type={showKey ? 'text' : 'password'}
              class="text-input"
              placeholder="Paste API Key (e.g. AIzaSy... / xai-... / sk-...)"
              bind:value={tempApiKey}
            />
            <button
              type="button"
              class="btn-toggle-eye"
              onclick={() => {
                soundState.playClick();
                showKey = !showKey;
              }}
              title={showKey ? 'Hide key' : 'Show key'}
            >
              {#if showKey}
                <EyeOff size={14} />
              {:else}
                <Eye size={14} />
              {/if}
            </button>
          </div>
          <span class="field-help">
            Keys are stored locally in your browser memory only (never written to disk or sent to external servers).
          </span>
        </div>

        <!-- Model Selector -->
        <div class="form-group">
          <label for="ai-model-select" class="form-label">
            <Cpu size={13} />
            <span>SELECT COMPLIANCE REASONING MODEL</span>
          </label>
          <select
            id="ai-model-select"
            class="select-input"
            bind:value={tempModelId}
          >
            <optgroup label="── Google Gemini ──">
              {#each googleModels as model}
                <option value={model.id}>{model.name} ({model.providerName})</option>
              {/each}
            </optgroup>
            <optgroup label="── xAI Grok ──">
              {#each xaiModels as model}
                <option value={model.id}>{model.name} ({model.providerName})</option>
              {/each}
            </optgroup>
            <optgroup label="── OpenAI GPT ──">
              {#each openaiModels as model}
                <option value={model.id}>{model.name} ({model.providerName})</option>
              {/each}
            </optgroup>
          </select>
        </div>

        <!-- Status Card -->
        <div class="status-box" class:valid={!!tempApiKey.trim()}>
          {#if tempApiKey.trim()}
            <span class="status-icon green"><CheckCircle2 size={16} /></span>
            <div class="status-info">
              <span class="status-title">API Key Configured</span>
              <span class="status-desc">Ready for AI-augmented compliance fact extraction</span>
            </div>
          {:else}
            <span class="status-icon yellow"><AlertCircle size={16} /></span>
            <div class="status-info">
              <span class="status-title">No API Key Specified</span>
              <span class="status-desc">When AI is ON, analysis will fall back to in-memory Prolog matching if key is missing.</span>
            </div>
          {/if}
        </div>
      </div>

      <div class="modal-footer">
        <button
          type="button"
          class="btn-secondary"
          onclick={() => {
            soundState.playClick();
            onClose();
          }}
        >
          CANCEL
        </button>
        <button
          type="button"
          class="btn-primary"
          onclick={handleSave}
        >
          SAVE & CLOSE
        </button>
      </div>
    </div>
  </div>
{/if}

<style>
  .modal-backdrop {
    position: fixed;
    top: 0;
    left: 0;
    width: 100vw;
    height: 100vh;
    background: rgba(4, 8, 14, 0.82);
    backdrop-filter: blur(4px);
    z-index: 9999;
    display: flex;
    align-items: center;
    justify-content: center;
    animation: fadeIn 0.2s ease;
  }

  @keyframes fadeIn {
    from { opacity: 0; }
    to { opacity: 1; }
  }

  .modal-card {
    background: #0d1724;
    border: 1px solid rgba(0, 240, 255, 0.25);
    border-radius: 6px;
    width: 90%;
    max-width: 480px;
    box-shadow: 0 0 30px rgba(0, 240, 255, 0.15);
    display: flex;
    flex-direction: column;
    overflow: hidden;
  }

  .modal-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 0.9rem 1.2rem;
    border-bottom: 1px solid rgba(255, 255, 255, 0.08);
    background: rgba(0, 240, 255, 0.03);
  }

  .modal-title-row {
    display: flex;
    align-items: center;
    gap: 8px;
  }

  .modal-title-row :global(.title-icon) {
    color: var(--cyan-primary);
  }

  .modal-title {
    font-family: var(--font-mono);
    font-size: 0.8rem;
    font-weight: 700;
    letter-spacing: 1.5px;
    color: #ffffff;
  }

  .btn-close {
    background: none;
    border: none;
    color: var(--text-muted);
    cursor: pointer;
    padding: 4px;
    border-radius: 4px;
    transition: all 0.2s ease;
  }

  .btn-close:hover {
    color: #ffffff;
    background: rgba(255, 255, 255, 0.1);
  }

  .modal-body {
    padding: 1.25rem;
    display: flex;
    flex-direction: column;
    gap: 1.2rem;
  }

  .form-group {
    display: flex;
    flex-direction: column;
    gap: 0.4rem;
  }

  .form-label {
    display: flex;
    align-items: center;
    gap: 6px;
    font-family: var(--font-mono);
    font-size: 0.68rem;
    font-weight: 700;
    letter-spacing: 1px;
    color: var(--cyan-muted);
  }

  .input-wrapper {
    position: relative;
    display: flex;
    align-items: center;
  }

  .text-input {
    width: 100%;
    padding: 9px 36px 9px 12px;
    background: rgba(4, 12, 22, 0.9);
    border: 1px solid rgba(0, 240, 255, 0.2);
    border-radius: 4px;
    color: #ffffff;
    font-family: var(--font-mono);
    font-size: 0.78rem;
    outline: none;
    transition: all 0.2s ease;
  }

  .text-input:focus {
    border-color: var(--cyan-primary);
    box-shadow: 0 0 10px rgba(0, 240, 255, 0.2);
  }

  .btn-toggle-eye {
    position: absolute;
    right: 8px;
    background: none;
    border: none;
    color: var(--text-muted);
    cursor: pointer;
    padding: 4px;
    border-radius: 3px;
  }

  .btn-toggle-eye:hover {
    color: #ffffff;
  }

  .select-input {
    width: 100%;
    padding: 9px 12px;
    background: rgba(4, 12, 22, 0.9);
    border: 1px solid rgba(0, 240, 255, 0.2);
    border-radius: 4px;
    color: #ffffff;
    font-family: var(--font-mono);
    font-size: 0.78rem;
    outline: none;
    cursor: pointer;
  }

  .select-input:focus {
    border-color: var(--cyan-primary);
  }

  .field-help {
    font-family: var(--font-body);
    font-size: 0.68rem;
    color: var(--text-muted);
    line-height: 1.3;
  }

  .status-box {
    display: flex;
    align-items: center;
    gap: 10px;
    padding: 10px 12px;
    background: rgba(255, 215, 0, 0.05);
    border: 1px solid rgba(255, 215, 0, 0.2);
    border-radius: 4px;
  }

  .status-box.valid {
    background: rgba(0, 255, 136, 0.05);
    border-color: rgba(0, 255, 136, 0.25);
  }

  .status-icon.green {
    color: #00ff88;
  }

  .status-icon.yellow {
    color: #ffd700;
  }

  .status-info {
    display: flex;
    flex-direction: column;
  }

  .status-title {
    font-family: var(--font-mono);
    font-size: 0.72rem;
    font-weight: 700;
    color: #ffffff;
  }

  .status-desc {
    font-family: var(--font-body);
    font-size: 0.68rem;
    color: var(--text-muted);
  }

  .modal-footer {
    display: flex;
    align-items: center;
    justify-content: flex-end;
    gap: 10px;
    padding: 0.85rem 1.2rem;
    border-top: 1px solid rgba(255, 255, 255, 0.08);
    background: rgba(0, 0, 0, 0.2);
  }

  .btn-secondary {
    background: none;
    border: 1px solid rgba(255, 255, 255, 0.15);
    color: var(--text-muted);
    font-family: var(--font-mono);
    font-size: 0.72rem;
    font-weight: 700;
    padding: 7px 14px;
    border-radius: 4px;
    cursor: pointer;
  }

  .btn-secondary:hover {
    color: #ffffff;
    border-color: rgba(255, 255, 255, 0.3);
  }

  .btn-primary {
    background: #132e35;
    border: 1px solid var(--cyan-primary);
    color: var(--cyan-primary);
    font-family: var(--font-mono);
    font-size: 0.72rem;
    font-weight: 700;
    letter-spacing: 1px;
    padding: 7px 16px;
    border-radius: 4px;
    cursor: pointer;
    transition: all 0.2s ease;
  }

  .btn-primary:hover {
    background: #1c3f48;
    box-shadow: 0 0 10px rgba(0, 240, 255, 0.3);
  }

  /* Light theme overrides */
  :global([data-theme="light"]) .modal-card {
    background: #c2c6ca;
    border-color: #a0a6ac;
    box-shadow: 0 0 20px rgba(0, 0, 0, 0.15);
  }

  :global([data-theme="light"]) .modal-title {
    color: #2d3a45;
  }

  :global([data-theme="light"]) .text-input,
  :global([data-theme="light"]) .select-input {
    background: #ffffff;
    border-color: #a0a6ac;
    color: #2d3a45;
  }

  :global([data-theme="light"]) .status-title {
    color: #2d3a45;
  }

  /* ─── Mobile Responsiveness ─── */
  @media (max-width: 640px) {
    .modal-card {
      width: 95%;
      max-height: 90vh;
    }

    .modal-header {
      padding: 0.75rem 1rem;
    }

    .modal-body {
      padding: 1rem;
      gap: 1rem;
      overflow-y: auto;
    }

    .modal-footer {
      padding: 0.75rem 1rem;
      flex-direction: column-reverse;
      width: 100%;
      gap: 8px;
    }

    .btn-secondary,
    .btn-primary {
      width: 100%;
      text-align: center;
      justify-content: center;
    }
  }
</style>
