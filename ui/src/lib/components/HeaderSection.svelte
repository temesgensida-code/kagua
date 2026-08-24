<script lang="ts">
  import type { Framework } from "$lib/data/sampleDocs";
  import SoundToggle from "./SoundToggle.svelte";
  import ThemeToggle from "./ThemeToggle.svelte";
  import AIToggle from "./AIToggle.svelte";
  import SettingsModal from "./SettingsModal.svelte";
  import { soundState } from "$lib/services/sound.svelte";

  interface Props {
    frameworks: Framework[];
    selectedDomain: string;
    onSelectDomain: (domain: string) => void;
    onToggleFramework: (id: string) => void;
    onToggleDomainFrameworks?: (domain: "labour" | "commercial") => void;
  }

  let {
    frameworks,
    selectedDomain = "auto",
    onSelectDomain,
    onToggleFramework,
    onToggleDomainFrameworks,
  }: Props = $props();

  let activeCount = $derived(frameworks.filter((f) => f.active).length);
  let isLabourActive = $derived(
    frameworks.filter((f) => f.domain === "labour").some((f) => f.active),
  );
  let isCommercialActive = $derived(
    frameworks.filter((f) => f.domain === "commercial").some((f) => f.active),
  );
  let isSettingsOpen = $state(false);
</script>

<header class="header-container">
  <div class="top-header-row">
    <!-- Tag line with vertical pink line accent -->
    <div class="tag-row">
      <span class="pink-line"></span>
      <span class="tag-text">
        ETHIOPIAN LEGAL COMPLIANCE ENGINE — LABOUR & COMMERCIAL CODE
      </span>
    </div>

    <!-- Top Right Controls: AI, Theme, Sound -->
    <div class="toggle-controls">
      <AIToggle onOpenSettings={() => (isSettingsOpen = true)} />
      <ThemeToggle />
      <SoundToggle />
    </div>
  </div>

  <SettingsModal
    isOpen={isSettingsOpen}
    onClose={() => (isSettingsOpen = false)}
  />

  <!-- Main Headline -->
  <h1 class="main-title">
    <span class="text-kagua">KAGUA</span>
  </h1>

  <!-- Description & Master Domain Buttons Row (Same Row!) -->
  <div class="description-row">
    <!-- Subtitle / Description -->
    <p class="description">
      Automated zero-persistence compliance engine for employment contracts and
      corporate governance under FDRE Labour Proclamation No. 1156/2019 and
      Commercial Code Proclamation No. 1243/2021. Powered by spaCy NER, In-Memory
      RAG, and SWI-Prolog legal reasoning.
    </p>

    <!-- Domain Master ON/OFF Buttons (Aligned on the same row) -->
    <div class="domain-master-toggles">
      <!-- Labour Law Domain Master Toggle -->
      <button
        type="button"
        class="domain-master-btn labour"
        class:active={isLabourActive}
        onclick={() => {
          onToggleDomainFrameworks?.("labour");
          soundState.playClick();
        }}
        title="Turn ON/OFF all Labour Law frameworks (Proc. 1156/2019)"
      >
        <div class="master-btn-info">
          <span class="master-title">LABOUR LAW DOMAIN</span>
          <span class="master-desc">Toggles all 6 Labour Law rules</span>
        </div>
        <span class="master-badge" class:on={isLabourActive}>
          {isLabourActive ? "ON" : "OFF"}
        </span>
      </button>

      <!-- Commercial Code Domain Master Toggle -->
      <button
        type="button"
        class="domain-master-btn commercial"
        class:active={isCommercialActive}
        onclick={() => {
          onToggleDomainFrameworks?.("commercial");
          soundState.playClick();
        }}
        title="Turn ON/OFF all Commercial Code frameworks (Proc. 1243/2021)"
      >
        <div class="master-btn-info">
          <span class="master-title">COMMERCIAL CODE DOMAIN</span>
          <span class="master-desc">Toggles all 3 Commercial Code rules</span>
        </div>
        <span class="master-badge" class:on={isCommercialActive}>
          {isCommercialActive ? "ON" : "OFF"}
        </span>
      </button>
    </div>
  </div>

  <!-- Active Framework Toggles (With ON/OFF per framework item) -->
  <div class="frameworks-section">
    <div class="frameworks-label">
      <span class="bullet">•</span> FRAMEWORK DOMAIN RULES
      <span class="count">({activeCount}/{frameworks.length} ACTIVE)</span>
    </div>

    <div class="badges-row">
      {#each frameworks as fw (fw.id)}
        <button
          type="button"
          class="framework-toggle-btn"
          class:inactive={!fw.active}
          style="
            --fw-color: {fw.color};
            --fw-bg: {fw.bg};
            --fw-border: {fw.border};
            --fw-glow: {fw.glow};
            --fw-light-bg: {fw.lightBg};
            --fw-light-border: {fw.lightBorder};
            --fw-light-color: {fw.lightColor};
          "
          onclick={() => {
            onToggleFramework(fw.id);
            soundState.playClick();
          }}
          title="{fw.description} — Click to turn {fw.active ? 'OFF' : 'ON'}"
        >
          <span class="badge-dot"></span>
          <span class="badge-name">{fw.name}</span>
          <span class="fw-toggle-badge" class:on={fw.active}>
            {fw.active ? "ON" : "OFF"}
          </span>
        </button>
      {/each}
    </div>
  </div>
</header>

<style>
  .header-container {
    width: 100%;
    margin-bottom: 0.5rem;
    text-align: left;
  }

  .top-header-row {
    display: flex;
    align-items: center;
    justify-content: space-between;
    width: 100%;
    margin-bottom: 0.25rem;
    gap: 1rem;
  }

  .tag-row {
    display: flex;
    align-items: center;
    gap: 8px;
  }

  .pink-line {
    display: inline-block;
    width: 3px;
    height: 12px;
    background: #ff2a70;
    box-shadow: 0 0 8px rgba(255, 42, 112, 0.7);
    border-radius: 1px;
  }

  .tag-text {
    font-family: var(--font-mono);
    font-size: 0.7rem;
    font-weight: 600;
    letter-spacing: 2.5px;
    color: var(--cyan-primary);
    text-shadow: 0 0 10px rgba(0, 240, 255, 0.4);
    text-transform: uppercase;
  }

  .toggle-controls {
    display: flex;
    align-items: center;
    gap: 8px;
  }

  .main-title {
    margin-bottom: 0.2rem;
    line-height: 1.1;
  }

  .text-kagua {
    font-family: var(--font-title);
    font-size: 1.75rem;
    font-weight: 800;
    letter-spacing: 4px;
    color: #ffffff;
    text-shadow: 0 0 15px rgba(0, 240, 255, 0.4);
  }

  /* Description + Domain Master Buttons Row (Placed on the same row!) */
  .description-row {
    display: flex;
    align-items: flex-start;
    justify-content: space-between;
    gap: 1.25rem;
    margin-bottom: 0.6rem;
    width: 100%;
  }

  .description {
    font-family: var(--font-body);
    font-size: 0.88rem;
    line-height: 1.5;
    color: var(--text-muted);
    max-width: 620px;
    margin-bottom: 0;
  }

  .domain-master-toggles {
    display: flex;
    flex-direction: column;
    gap: 5px;
    align-items: flex-end;
    flex-shrink: 0;
  }

  .domain-master-btn {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 12px;
    padding: 5px 9px;
    border-radius: 5px;
    cursor: pointer;
    font-family: var(--font-mono);
    transition: all 0.2s cubic-bezier(0.16, 1, 0.3, 1);
    min-width: 215px;
    user-select: none;
    text-align: left;
  }

  .domain-master-btn.labour {
    background: rgba(30, 41, 59, 0.85);
    border: 1px solid #475569;
    color: #cbd5e1;
  }

  .domain-master-btn.commercial {
    background: rgba(15, 23, 42, 0.85);
    border: 1px solid #334155;
    color: #94a3b8;
  }

  .domain-master-btn:hover {
    transform: translateY(-1px);
    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.3);
  }

  .domain-master-btn:not(.active) {
    opacity: 0.45;
    border-color: rgba(255, 255, 255, 0.1);
    background: rgba(15, 23, 42, 0.5);
    color: #64748b;
  }

  .master-btn-info {
    display: flex;
    flex-direction: column;
    gap: 1px;
  }

  .master-title {
    font-size: 0.63rem;
    font-weight: 800;
    letter-spacing: 1px;
  }

  .master-desc {
    font-size: 0.54rem;
    color: var(--text-muted);
    font-weight: 500;
  }

  .master-badge {
    font-size: 0.58rem;
    font-weight: 800;
    padding: 2px 6px;
    border-radius: 3px;
    background: rgba(255, 255, 255, 0.08);
    color: #64748b;
    transition: all 0.2s ease;
  }

  .master-badge.on {
    background: rgba(148, 163, 184, 0.3);
    color: #ffffff;
    box-shadow: 0 0 8px rgba(148, 163, 184, 0.3);
  }

  .frameworks-section {
    display: flex;
    flex-direction: column;
    gap: 0.4rem;
  }

  .frameworks-label {
    font-family: var(--font-mono);
    font-size: 0.66rem;
    font-weight: 600;
    letter-spacing: 2px;
    color: #8a9aa8;
    text-transform: uppercase;
    display: flex;
    align-items: center;
    gap: 6px;
  }

  .bullet {
    color: var(--cyan-primary);
    font-size: 0.85rem;
  }

  .count {
    color: #64748b;
    font-size: 0.63rem;
  }

  .badges-row {
    display: flex;
    flex-wrap: wrap;
    gap: 0.45rem;
    align-items: center;
  }

  .framework-toggle-btn {
    position: relative;
    display: inline-flex;
    align-items: center;
    gap: 7px;
    padding: 5px 11px;
    background: var(--fw-bg);
    border: 1px solid var(--fw-border);
    border-radius: 5px;
    color: var(--fw-color);
    font-family: var(--font-mono);
    font-size: 0.71rem;
    font-weight: 700;
    letter-spacing: 0.8px;
    cursor: pointer;
    transition: all 0.2s cubic-bezier(0.16, 1, 0.3, 1);
    box-shadow: var(--fw-glow);
    user-select: none;
  }

  .framework-toggle-btn:hover {
    transform: translateY(-2px);
    border-color: var(--fw-color);
  }

  .framework-toggle-btn.inactive {
    opacity: 0.45;
    border-color: rgba(255, 255, 255, 0.1);
    background: rgba(15, 23, 42, 0.6);
    color: #64748b;
    box-shadow: none;
    filter: grayscale(0.5);
  }

  .badge-dot {
    width: 5px;
    height: 5px;
    background-color: currentColor;
    border-radius: 50%;
    box-shadow: 0 0 6px currentColor;
  }

  .fw-toggle-badge {
    font-size: 0.58rem;
    font-weight: 800;
    letter-spacing: 0.5px;
    padding: 2px 6px;
    border-radius: 3px;
    background: rgba(255, 255, 255, 0.08);
    color: #94a3b8;
    transition: all 0.2s ease;
  }

  .fw-toggle-badge.on {
    background: rgba(148, 163, 184, 0.25);
    color: #f1f5f9;
    box-shadow: 0 0 8px rgba(148, 163, 184, 0.2);
  }

  .framework-toggle-btn.inactive .fw-toggle-badge {
    background: rgba(255, 255, 255, 0.05);
    color: #475569;
  }

  /* ─── Light Mode Header Overrides ─── */
  :global([data-theme="light"]) .tag-text {
    color: #4a5a68;
    text-shadow: none;
  }

  :global([data-theme="light"]) .pink-line {
    box-shadow: none;
  }

  :global([data-theme="light"]) .text-kagua {
    color: #2d3a45;
    text-shadow: none;
  }

  :global([data-theme="light"]) .description {
    color: #5a6a78;
  }

  :global([data-theme="light"]) .frameworks-label {
    color: #5a6a78;
  }

  :global([data-theme="light"]) .bullet {
    color: #69818d;
  }

  :global([data-theme="light"]) .count {
    color: #8a9aa8;
  }

  /* Distinct Light Mode Framework Button Colors */
  :global([data-theme="light"]) .framework-toggle-btn {
    background: var(--fw-light-bg);
    border-color: var(--fw-light-border);
    color: var(--fw-light-color);
    box-shadow: 0 2px 6px rgba(0, 0, 0, 0.06);
  }

  :global([data-theme="light"]) .framework-toggle-btn.inactive {
    background: #e2e8f0;
    border-color: #cbd5e1;
    color: #94a3b8;
    opacity: 0.55;
    filter: none;
    box-shadow: none;
  }

  :global([data-theme="light"]) .fw-toggle-badge {
    background: rgba(0, 0, 0, 0.12);
    color: var(--fw-light-color);
  }

  :global([data-theme="light"]) .fw-toggle-badge.on {
    background: rgba(0, 0, 0, 0.2);
    color: #000000;
  }

  :global([data-theme="light"])
    .framework-toggle-btn.inactive
    .fw-toggle-badge {
    background: rgba(0, 0, 0, 0.08);
    color: #94a3b8;
  }

  /* Light Mode Domain Master Buttons */
  :global([data-theme="light"]) .domain-master-btn.labour {
    background: #e2e8f0;
    border-color: #94a3b8;
    color: #0f172a;
  }

  :global([data-theme="light"]) .domain-master-btn.commercial {
    background: #cbd5e1;
    border-color: #64748b;
    color: #020617;
  }

  :global([data-theme="light"]) .domain-master-btn:not(.active) {
    background: #f1f5f9;
    border-color: #cbd5e1;
    color: #94a3b8;
    opacity: 0.6;
  }

  :global([data-theme="light"]) .master-badge {
    background: rgba(0, 0, 0, 0.1);
    color: #64748b;
  }

  :global([data-theme="light"]) .master-badge.on {
    background: rgba(0, 0, 0, 0.2);
    color: #0f172a;
    box-shadow: none;
  }

  :global([data-theme="light"]) .master-desc {
    color: #64748b;
  }
</style>
