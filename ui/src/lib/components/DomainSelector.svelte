<script lang="ts">
  import { soundState } from '$lib/services/sound.svelte';
  import { ShieldAlert, Building2, Cpu } from '@lucide/svelte';

  interface Props {
    selectedDomain: string;
    onSelectDomain: (domain: string) => void;
  }

  let { selectedDomain = 'auto', onSelectDomain }: Props = $props();

  const DOMAINS = [
    {
      id: 'auto',
      title: 'AUTO-DETECT',
      subtitle: 'Dynamic RAG Dispatcher',
      description: 'Intelligent automatic domain routing based on document text & spaCy NER clause detection',
      icon: Cpu,
      accent: '#00f0ff',
      badge: 'RECOMMENDED'
    },
    {
      id: 'ethiopian_labour_proclamation',
      title: 'LABOUR LAW (1156/2019)',
      subtitle: 'Employment Compliance',
      description: 'Proclamation No. 1156/2019 — Probation, working hours, leave entitlements, severance & workplace protections',
      icon: ShieldAlert,
      accent: '#afb3b7',
      badge: 'PROCLAMATION 1156'
    },
    {
      id: 'commercial_code',
      title: 'COMMERCIAL CODE (1243/2021)',
      subtitle: 'Corporate Governance',
      description: 'Proclamation No. 1243/2021 — Share Companies, PLCs, capital thresholds, board composition & legal reserves',
      icon: Building2,
      accent: '#00ff88',
      badge: 'PROCLAMATION 1243'
    }
  ];

  function selectDomain(id: string) {
    soundState.playClick();
    onSelectDomain(id);
  }
</script>

<div class="domain-selector-container">
  <div class="selector-header">
    <span class="bullet-dot"></span>
    <span class="selector-label">TARGET COMPLIANCE DOMAIN</span>
    <span class="selector-hint">Select statutory proclamation for reasoning engine</span>
  </div>

  <div class="domain-buttons-grid">
    {#each DOMAINS as item}
      <button
        type="button"
        class="domain-card-btn"
        class:active={selectedDomain === item.id}
        style="--accent-color: {item.accent};"
        onclick={() => selectDomain(item.id)}
      >
        <div class="card-top">
          <div class="icon-wrap">
            <item.icon size={16} class="domain-icon" />
          </div>
          <span class="domain-badge">{item.badge}</span>
        </div>

        <div class="domain-info">
          <h4 class="domain-title">{item.title}</h4>
          <span class="domain-subtitle">{item.subtitle}</span>
          <p class="domain-desc">{item.description}</p>
        </div>

        {#if selectedDomain === item.id}
          <div class="active-indicator">
            <span class="indicator-pulse"></span>
            ACTIVE
          </div>
        {/if}
      </button>
    {/each}
  </div>
</div>

<style>
  .domain-selector-container {
    width: 100%;
    display: flex;
    flex-direction: column;
    gap: 0.65rem;
    margin-bottom: 0.75rem;
  }

  .selector-header {
    display: flex;
    align-items: center;
    gap: 8px;
  }

  .bullet-dot {
    width: 6px;
    height: 6px;
    background: var(--cyan-primary);
    border-radius: 50%;
    box-shadow: 0 0 8px var(--cyan-primary);
  }

  .selector-label {
    font-family: var(--font-mono);
    font-size: 0.7rem;
    font-weight: 700;
    letter-spacing: 2px;
    color: var(--cyan-primary);
    text-transform: uppercase;
  }

  .selector-hint {
    font-family: var(--font-mono);
    font-size: 0.62rem;
    color: var(--text-muted);
    letter-spacing: 1px;
    margin-left: auto;
  }

  .domain-buttons-grid {
    display: grid;
    grid-template-columns: repeat(3, 1fr);
    gap: 0.75rem;
  }

  @media (max-width: 840px) {
    .domain-buttons-grid {
      grid-template-columns: 1fr;
    }
  }

  .domain-card-btn {
    position: relative;
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
    background: rgba(13, 23, 36, 0.75);
    border: 1px solid rgba(255, 255, 255, 0.08);
    border-radius: 6px;
    padding: 0.85rem 1rem;
    text-align: left;
    cursor: pointer;
    transition: all 0.22s cubic-bezier(0.16, 1, 0.3, 1);
    box-shadow: 0 4px 15px rgba(0, 0, 0, 0.2);
  }

  .domain-card-btn:hover {
    background: rgba(19, 46, 53, 0.4);
    border-color: rgba(0, 240, 255, 0.3);
    transform: translateY(-2px);
  }

  .domain-card-btn.active {
    background: rgba(13, 34, 48, 0.95);
    border-color: var(--accent-color);
    box-shadow: 0 0 16px rgba(0, 240, 255, 0.15), inset 0 0 12px rgba(0, 240, 255, 0.05);
  }

  .card-top {
    display: flex;
    align-items: center;
    justify-content: space-between;
    width: 100%;
  }

  .icon-wrap {
    width: 28px;
    height: 28px;
    background: rgba(255, 255, 255, 0.05);
    border-radius: 4px;
    display: flex;
    align-items: center;
    justify-content: center;
    color: var(--accent-color);
  }

  .domain-card-btn.active .icon-wrap {
    background: rgba(0, 240, 255, 0.15);
    color: var(--accent-color);
  }

  .domain-badge {
    font-family: var(--font-mono);
    font-size: 0.58rem;
    font-weight: 700;
    letter-spacing: 1px;
    color: #6b8296;
    background: rgba(255, 255, 255, 0.04);
    border: 1px solid rgba(255, 255, 255, 0.08);
    padding: 2px 6px;
    border-radius: 3px;
  }

  .domain-card-btn.active .domain-badge {
    color: var(--accent-color);
    border-color: var(--accent-color);
    background: rgba(0, 240, 255, 0.1);
  }

  .domain-info {
    display: flex;
    flex-direction: column;
    gap: 2px;
  }

  .domain-title {
    font-family: var(--font-mono);
    font-size: 0.76rem;
    font-weight: 800;
    letter-spacing: 1px;
    color: #ffffff;
  }

  .domain-subtitle {
    font-family: var(--font-mono);
    font-size: 0.64rem;
    color: var(--cyan-muted);
    font-weight: 600;
  }

  .domain-desc {
    font-family: var(--font-body);
    font-size: 0.68rem;
    line-height: 1.35;
    color: var(--text-muted);
    margin-top: 4px;
  }

  .active-indicator {
    position: absolute;
    top: 8px;
    right: 8px;
    display: flex;
    align-items: center;
    gap: 4px;
    font-family: var(--font-mono);
    font-size: 0.55rem;
    font-weight: 800;
    color: var(--accent-color);
    letter-spacing: 1px;
  }

  .indicator-pulse {
    width: 5px;
    height: 5px;
    background: var(--accent-color);
    border-radius: 50%;
    box-shadow: 0 0 6px var(--accent-color);
    animation: pulse 1.5s infinite;
  }

  @keyframes pulse {
    0% { opacity: 0.4; transform: scale(0.9); }
    50% { opacity: 1; transform: scale(1.3); }
    100% { opacity: 0.4; transform: scale(0.9); }
  }

  /* Light mode overrides */
  :global([data-theme="light"]) .domain-card-btn {
    background: #c2c6ca;
    border-color: #8a9aa8;
    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.08);
  }

  :global([data-theme="light"]) .domain-card-btn:hover {
    background: #b0b6bc;
    border-color: #69818d;
  }

  :global([data-theme="light"]) .domain-card-btn.active {
    background: #ffffff;
    border-color: #0088cc;
  }

  :global([data-theme="light"]) .domain-title {
    color: #2d3a45;
  }

  :global([data-theme="light"]) .domain-subtitle {
    color: #0088cc;
  }

  :global([data-theme="light"]) .domain-desc {
    color: #5a6a78;
  }
</style>
