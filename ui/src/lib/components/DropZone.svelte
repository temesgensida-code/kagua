<script lang="ts">
  import type { ProgressEvent } from "$lib/services/api";
  import { soundState } from '$lib/services/sound.svelte';
  import { Download } from '@lucide/svelte';

  interface Props {
    isScanning: boolean;
    progressEvents: ProgressEvent[];
    currentStageMessage: string;
    selectedDomain?: string;
    onFileSelected: (
      fileData: Blob | File,
      filename: string,
      sampleContent?: string,
      sampleDomain?: string
    ) => void;
  }

  let {
    isScanning = false,
    progressEvents = [],
    currentStageMessage = "",
    selectedDomain = "auto",
    onFileSelected,
  }: Props = $props();

  let isDragging = $state(false);
  let fileInput: HTMLInputElement;

  function handleDragOver(e: DragEvent) {
    e.preventDefault();
    isDragging = true;
  }

  function handleDragLeave(e: DragEvent) {
    e.preventDefault();
    isDragging = false;
  }

  function handleDrop(e: DragEvent) {
    e.preventDefault();
    isDragging = false;
    if (e.dataTransfer?.files && e.dataTransfer.files.length > 0) {
      const file = e.dataTransfer.files[0];
      soundState.playBrowse();
      onFileSelected(file, file.name);
    }
  }

  function handleFileInputChange(e: Event) {
    const input = e.target as HTMLInputElement;
    if (input.files && input.files.length > 0) {
      const file = input.files[0];
      onFileSelected(file, file.name);
      input.value = "";
    }
  }

  function triggerBrowse() {
    soundState.playBrowse();
    fileInput?.click();
  }
</script>

<div class="dropzone-wrapper">
  <div
    class="dropzone-panel"
    class:dragging={isDragging}
    class:scanning={isScanning}
    ondragover={handleDragOver}
    ondragleave={handleDragLeave}
    ondrop={handleDrop}
    role="region"
    aria-label="Document Drop Zone"
  >
    <input
      type="file"
      bind:this={fileInput}
      onchange={handleFileInputChange}
      accept=".txt,.pdf,.doc,.docx,.md"
      class="hidden-input"
    />

    <!-- Real-time WebSocket Scanning Progress Overlay -->
    {#if isScanning}
      <div class="scan-laser"></div>
      <div class="scanning-hud">
        <div class="spinner"></div>
        <span class="scan-stage"
          >{currentStageMessage ||
            "CONNECTING TO RUST ANALYSIS PIPELINE..."}</span
        >

        <!-- Step-by-Step Progress Events Log -->
        <div class="progress-log">
          {#each progressEvents.slice(-4) as evt}
            <div class="log-line">
              <span class="log-stage">[{evt.stage}]</span>
              <span class="log-msg">{evt.message}</span>
            </div>
          {/each}
        </div>
      </div>
    {/if}

    <div class="dropzone-content" class:fade={isScanning}>
      <div class="icon-container">
        <svg
          class="doc-icon"
          viewBox="0 0 24 24"
          fill="none"
          xmlns="http://www.w3.org/2000/svg"
        >
          <path
            d="M14 2H6C4.89543 2 4 2.89543 4 4V20C4 21.1046 4.89543 22 6 22H18C19.1046 22 20 21.1046 20 20V8L14 2Z"
            stroke="#00f0ff"
            stroke-width="1.8"
            stroke-linecap="round"
            stroke-linejoin="round"
          />
          <path
            d="M14 2V8H20"
            stroke="#00f0ff"
            stroke-width="1.8"
            stroke-linecap="round"
            stroke-linejoin="round"
          />
          <path
            d="M12 17V11M12 11L9.5 13.5M12 11L14.5 13.5"
            stroke="#00f0ff"
            stroke-width="2"
            stroke-linecap="round"
            stroke-linejoin="round"
          />
        </svg>
      </div>

      <h2 class="drop-title">DROP DOCUMENT FOR COMPLIANCE REASONING</h2>

      <p class="file-specs">
        .TXT &bull; .PDF &bull; .DOC &bull; .DOCX &bull; .MD &mdash; IN-MEMORY
        STREAMING (NO DISK PERSISTENCE)
      </p>

      <button type="button" class="btn-browse" onclick={triggerBrowse}>
        BROWSE FILES
      </button>
    </div>
  </div>

  <!-- Sample PDF Download Bar -->
  <div class="sample-download-bar">
    <a
      href="/fictional_vacancy_30pages.pdf"
      download="fictional_vacancy_30pages.pdf"
      class="sample-download-btn"
      onclick={() => soundState.playClick()}
      title="Download sample PDF document (30 pages) to test compliance analysis"
    >
      <Download size={13} />
      <span class="sample-dl-text">Sample PDF file to test with</span>
      <span class="sample-dl-badge">PDF &bull; 30 PAGES</span>
    </a>
  </div>
</div>

<style>
  .dropzone-wrapper {
    width: 100%;
    max-width: 680px;
    margin: 0 auto;
    display: flex;
    flex-direction: column;
    gap: 0.75rem;
  }

  .dropzone-panel {
    position: relative;
    width: 100%;
    min-height: 270px;
    background: #0d1724;
    background: radial-gradient(circle at center, #111d2e 0%, #0b1420 100%);
    border: 2px dashed #1e434c;
    border-radius: 8px;
    padding: 2.2rem 1.5rem;
    display: flex;
    align-items: center;
    justify-content: center;
    box-shadow: 0 10px 30px rgba(0, 0, 0, 0.5);
    transition: all 0.25s ease;
    overflow: hidden;
  }

  .dropzone-panel:hover {
    border-color: #2a5863;
  }

  .dropzone-panel.dragging {
    border: 2px dashed #2a5863;
    background: rgba(19, 46, 53, 0.3);
  }

  .hidden-input {
    display: none;
  }

  .dropzone-content {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    text-align: center;
    z-index: 1;
    transition: opacity 0.3s ease;
  }

  .dropzone-content.fade {
    opacity: 0.15;
    pointer-events: none;
  }

  .icon-container {
    width: 60px;
    height: 60px;
    background: rgba(19, 46, 53, 0.25);
    border: 1px solid #1e434c;
    border-radius: 12px;
    display: flex;
    align-items: center;
    justify-content: center;
    margin-bottom: 1.25rem;
    transition:
      transform 0.3s ease,
      border-color 0.3s ease;
  }

  .dropzone-panel:hover .icon-container {
    transform: translateY(-4px);
    border-color: #2a5d6a;
  }

  .doc-icon {
    width: 30px;
    height: 30px;
  }

  .drop-title {
    font-family: var(--font-title);
    font-size: 1.1rem;
    font-weight: 700;
    letter-spacing: 2.5px;
    color: #ffffff;
    margin-bottom: 0.4rem;
  }

  .file-specs {
    font-family: var(--font-mono);
    font-size: 0.7rem;
    font-weight: 500;
    letter-spacing: 1.5px;
    color: var(--text-muted);
    margin-bottom: 1.5rem;
  }

  .btn-browse {
    font-family: var(--font-title);
    font-size: 0.85rem;
    font-weight: 700;
    letter-spacing: 2px;
    color: #e2f1f8;
    background: #132E35;
    border: 1px solid #1e434c;
    border-radius: 4px;
    padding: 11px 32px;
    cursor: pointer;
    transition: all 0.2s cubic-bezier(0.16, 1, 0.3, 1);
    box-shadow: 0 2px 2px rgba(0, 0, 0, 0.4);
  }

  .btn-browse:hover {
    background: #1c3f48;
    border-color: #2a5863;
    transform: translateY(-2px) scale(1.02);
  }

  /* Scan Laser Animation */
  .scan-laser {
    position: absolute;
    left: 0;
    right: 0;
    height: 3px;
    background: linear-gradient(
      90deg,
      transparent 0%,
      var(--cyan-primary) 50%,
      transparent 100%
    );
    box-shadow:
      0 0 15px var(--cyan-primary),
      0 0 30px var(--cyan-primary);
    animation: scanline 1.8s ease-in-out infinite alternate;
    z-index: 5;
  }

  @keyframes scanline {
    0% {
      top: 5%;
    }
    100% {
      top: 95%;
    }
  }

  .scanning-hud {
    position: absolute;
    z-index: 6;
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 12px;
    max-width: 90%;
  }

  .spinner {
    width: 36px;
    height: 36px;
    border: 3px solid rgba(0, 240, 255, 0.2);
    border-top-color: var(--cyan-primary);
    border-radius: 50%;
    animation: spin 0.8s linear infinite;
  }

  @keyframes spin {
    to {
      transform: rotate(360deg);
    }
  }

  .scan-stage {
    font-family: var(--font-mono);
    font-size: 0.82rem;
    font-weight: 700;
    letter-spacing: 2px;
    color: var(--cyan-primary);
    text-shadow: 0 0 10px rgba(0, 240, 255, 0.8);
    text-align: center;
  }

  .progress-log {
    display: flex;
    flex-direction: column;
    gap: 4px;
    background: rgba(4, 8, 14, 0.85);
    border: 1px solid rgba(0, 240, 255, 0.2);
    border-radius: 4px;
    padding: 8px 14px;
    max-width: 500px;
  }

  .log-line {
    font-family: var(--font-mono);
    font-size: 0.68rem;
    display: flex;
    align-items: center;
    gap: 8px;
  }

  .log-stage {
    color: var(--cyan-primary);
    font-weight: 700;
  }

  .log-msg {
    color: var(--text-muted);
  }

  /* Sample Download Bar */
  .sample-download-bar {
    display: flex;
    align-items: center;
    justify-content: center;
    padding: 0.35rem 0.25rem;
  }

  .sample-download-btn {
    display: inline-flex;
    align-items: center;
    gap: 6px;
    padding: 5px 12px;
    background: rgba(14, 25, 38, 0.85);
    border: 1px solid rgba(0, 240, 255, 0.25);
    border-radius: 5px;
    color: var(--cyan-primary);
    font-family: var(--font-mono);
    font-size: 0.68rem;
    font-weight: 600;
    text-decoration: none;
    cursor: pointer;
    transition: all 0.2s cubic-bezier(0.16, 1, 0.3, 1);
    box-shadow: 0 2px 2px rgba(0, 0, 0, 0.4);
  }

  .sample-download-btn:hover {
    background: rgba(0, 240, 255, 0.12);
    border-color: var(--cyan-primary);
    transform: translateY(-1px);
    box-shadow: 0 4px 8px rgba(0, 240, 255, 0.2);
  }

  .sample-dl-text {
    letter-spacing: 0.5px;
  }

  .sample-dl-badge {
    font-size: 0.54rem;
    font-weight: 800;
    padding: 1px 5px;
    border-radius: 3px;
    background: rgba(0, 240, 255, 0.15);
    color: var(--cyan-primary);
    letter-spacing: 0.5px;
  }

  /* ─── Light Mode Overrides ─── */
  :global([data-theme="light"]) .dropzone-panel {
    background: #c2c6ca;
    border-color: #8a9aa8;
    box-shadow: 0 10px 30px rgba(0, 0, 0, 0.12);
  }

  :global([data-theme="light"]) .dropzone-panel:hover {
    border-color: #69818d;
  }

  :global([data-theme="light"]) .icon-container {
    background: rgba(105, 129, 141, 0.15);
    border-color: #8a9aa8;
  }

  :global([data-theme="light"]) .doc-icon path {
    stroke: #69818d;
  }

  :global([data-theme="light"]) .drop-title {
    color: #2d3a45;
  }

  :global([data-theme="light"]) .file-specs {
    color: #5a6a78;
  }

  :global([data-theme="light"]) .btn-browse {
    background: #69818d;
    border-color: #8a9aa8;
    color: #ffffff;
    box-shadow: 0 2px 2px rgba(0, 0, 0, 0.4);
  }

  :global([data-theme="light"]) .btn-browse:hover {
    background: #5a7380;
    border-color: #69818d;
  }

  :global([data-theme="light"]) .sample-download-btn {
    background: #69818d;
    border: 1px solid #8a9aa8;
    color: #ffffff;
    box-shadow: 0 2px 2px rgba(0, 0, 0, 0.4);
  }

  :global([data-theme="light"]) .sample-download-btn:hover {
    background: #5a7380;
    border-color: #69818d;
    color: #ffffff;
  }

  :global([data-theme="light"]) .sample-dl-badge {
    background: rgba(255, 255, 255, 0.25);
    color: #ffffff;
  }

  /* ─── Mobile Responsiveness ─── */
  @media (max-width: 640px) {
    .dropzone-wrapper {
      gap: 0.5rem;
    }

    .dropzone-panel {
      padding: 1.5rem 1rem;
      min-height: 220px;
    }

    .icon-container {
      width: 48px;
      height: 48px;
      margin-bottom: 0.85rem;
    }

    .doc-icon {
      width: 24px;
      height: 24px;
    }

    .drop-title {
      font-size: 0.95rem;
      letter-spacing: 1.5px;
      line-height: 1.3;
    }

    .file-specs {
      font-size: 0.62rem;
      letter-spacing: 1px;
      margin-bottom: 1rem;
    }

    .btn-browse {
      width: 100%;
      max-width: 260px;
      padding: 10px 20px;
      font-size: 0.8rem;
    }

    .sample-download-btn {
      width: 100%;
      justify-content: center;
      padding: 4px 10px;
      font-size: 0.64rem;
      flex-wrap: wrap;
    }

    .progress-log {
      max-width: 100%;
      padding: 6px 10px;
    }
  }
</style>
