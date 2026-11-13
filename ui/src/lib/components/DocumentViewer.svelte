<script lang="ts">
  import { onMount } from 'svelte';
  import type { MappedViolation } from '$lib/services/api';
  import { soundState } from '$lib/services/sound.svelte';
  import { FileText, Maximize2, Minimize2, AlertTriangle, Minus, Plus } from '@lucide/svelte';

  let pdfjsLib = $state<any>(null);

  onMount(async () => {
    if (typeof window !== 'undefined') {
      const pdfModule = await import('pdfjs-dist');
      pdfjsLib = pdfModule;
      pdfjsLib.GlobalWorkerOptions.workerSrc = `https://cdnjs.cloudflare.com/ajax/libs/pdf.js/${pdfjsLib.version}/pdf.worker.min.mjs`;
    }
  });

  interface Props {
    filename: string;
    fileBlob?: Blob | File;
    rawText: string;
    violations: MappedViolation[];
    activeViolationIndex: number | null;
    onSelectViolation: (index: number) => void;
  }

  let { filename, fileBlob, rawText, violations, activeViolationIndex, onSelectViolation }: Props = $props();

  let viewerContainer = $state<HTMLDivElement | undefined>(undefined);
  let pdfCanvasContainer = $state<HTMLDivElement | undefined>(undefined);

  let isPdf = $derived(
    Boolean((fileBlob && fileBlob.type.includes('pdf')) || filename.toLowerCase().endsWith('.pdf'))
  );

  let numPages = $state(0);
  let currentPage = $state(1);
  let zoomLevel = $state(1.15);
  let pdfDoc = $state<any>(null);
  let isPdfLoading = $state(false);
  let pdfError = $state<string | null>(null);
  let pageTexts = $state<Record<number, string>>({});
  let isExpanded = $state(false);

  let pulseIndex = $state<number | null>(null);
  let pulseTimer: any = null;
  let highlightedPage = $state<number | null>(null);

  onMount(() => {
    function handleKeyDown(e: KeyboardEvent) {
      if (e.key === 'Escape' && isExpanded) {
        isExpanded = false;
      }
    }
    window.addEventListener('keydown', handleKeyDown);
    return () => {
      window.removeEventListener('keydown', handleKeyDown);
    };
  });

  // Load PDF when fileBlob or isPdf changes
  $effect(() => {
    if (isPdf && fileBlob) {
      if (!pdfjsLib) {
        import('pdfjs-dist').then(mod => {
          pdfjsLib = mod;
          pdfjsLib.GlobalWorkerOptions.workerSrc = `https://cdnjs.cloudflare.com/ajax/libs/pdf.js/${pdfjsLib.version}/pdf.worker.min.mjs`;
          loadPdf(fileBlob);
        });
      } else {
        loadPdf(fileBlob);
      }
    }
  });

  async function loadPdf(blob: Blob | File) {
    if (!pdfjsLib) return;
    try {
      isPdfLoading = true;
      pdfError = null;
      pageTexts = {};
      const arrayBuffer = await blob.arrayBuffer();
      const loadingTask = pdfjsLib.getDocument({ data: arrayBuffer });
      pdfDoc = await loadingTask.promise;
      numPages = pdfDoc.numPages;

      // Extract text content per page for violation snippet matching
      for (let pageNum = 1; pageNum <= numPages; pageNum++) {
        const page = await pdfDoc.getPage(pageNum);
        const textContent = await page.getTextContent();
        const textItems = textContent.items.map((item: any) => item.str).join(' ');
        pageTexts[pageNum] = textItems.toLowerCase();
      }

      await renderPdfPages();
    } catch (err: any) {
      console.error('PDF rendering error:', err);
      pdfError = `Could not render PDF canvas: ${err.message}`;
    } finally {
      isPdfLoading = false;
    }
  }

  async function renderPdfPages() {
    if (!pdfDoc || !pdfCanvasContainer) return;
    pdfCanvasContainer.innerHTML = '';

    for (let pageNum = 1; pageNum <= numPages; pageNum++) {
      const page = await pdfDoc.getPage(pageNum);
      const viewport = page.getViewport({ scale: zoomLevel });

      const pageWrapper = document.createElement('div');
      pageWrapper.className = 'pdf-page-wrapper';
      pageWrapper.dataset.pageNum = String(pageNum);

      const canvas = document.createElement('canvas');
      canvas.className = 'pdf-canvas';
      const context = canvas.getContext('2d');
      canvas.height = viewport.height;
      canvas.width = viewport.width;

      const pageBadge = document.createElement('div');
      pageBadge.className = 'pdf-page-number-badge';
      pageBadge.innerText = `PAGE ${pageNum} OF ${numPages}`;

      pageWrapper.appendChild(canvas);
      pageWrapper.appendChild(pageBadge);
      pdfCanvasContainer.appendChild(pageWrapper);

      const renderContext = {
        canvasContext: context!,
        viewport: viewport
      };
      await page.render(renderContext).promise;
    }
  }

  export function jumpToPage(pageNum: number) {
    if (!pdfCanvasContainer || !viewerContainer) return;
    const clampedPage = Math.max(1, Math.min(numPages, pageNum));
    currentPage = clampedPage;
    highlightedPage = clampedPage;
    const targetPageEl = pdfCanvasContainer.querySelector(
      `[data-page-num="${clampedPage}"]`
    ) as HTMLElement | null;
    if (targetPageEl) {
      const containerTop = pdfCanvasContainer.offsetTop;
      const pageRelativeTop = targetPageEl.offsetTop;
      const scrollTarget = containerTop + pageRelativeTop - 20;
      viewerContainer.scrollTo({
        top: scrollTarget,
        behavior: 'smooth'
      });
      targetPageEl.classList.add('pulse-page-highlight');
      setTimeout(() => {
        targetPageEl.classList.remove('pulse-page-highlight');
      }, 2500);
    }
  }

  // Handle active violation scroll and temporary highlight pulse
  $effect(() => {
    if (activeViolationIndex !== null && violations[activeViolationIndex]) {
      const v = violations[activeViolationIndex];

      // If violation is an omission, do not scroll or pulse document pages
      if (v.is_omission) {
        return;
      }

      // Trigger temporary 2.5s pulse highlight
      pulseIndex = activeViolationIndex;
      if (pulseTimer) clearTimeout(pulseTimer);
      pulseTimer = setTimeout(() => {
        pulseIndex = null;
        highlightedPage = null;
      }, 2500);

      if (pdfDoc && pdfCanvasContainer) {
        let bestPage = 1;

        if (typeof v.page_number === 'number' && v.page_number >= 1) {
          bestPage = Math.min(Math.max(1, v.page_number), numPages);
        } else {
          // --- Fallback Page Detection if page_number is not set ---
          const searchTerms: string[] = [];

          if (v.snippet) {
            const snippetWords = v.snippet.toLowerCase().replace(/[^\w\s]/g, ' ').trim();
            if (snippetWords.length > 10) searchTerms.push(snippetWords.slice(0, 60));
            if (snippetWords.length > 6) searchTerms.push(snippetWords.slice(0, 30));
          }

          if (v.snippet) {
            const words = v.snippet.toLowerCase()
              .replace(/[^\w\s]/g, ' ')
              .split(/\s+/)
              .filter(w => w.length > 5)
              .slice(0, 5);
            searchTerms.push(...words);
          }

          if (v.title) {
            const titleWords = v.title.toLowerCase()
              .replace(/[^\w\s]/g, ' ')
              .split(/\s+/)
              .filter(w => w.length > 4);
            searchTerms.push(...titleWords);
          }

          if (v.description) {
            const descWords = v.description.toLowerCase()
              .replace(/[^\w\s]/g, ' ')
              .split(/\s+/)
              .filter(w => w.length > 6)
              .slice(0, 5);
            searchTerms.push(...descWords);
          }

          let bestScore = -1;

          for (let p = 1; p <= numPages; p++) {
            const pageText = pageTexts[p];
            if (!pageText) continue;

            let score = 0;
            for (const term of searchTerms) {
              if (pageText.includes(term)) {
                score += term.length;
              }
            }
            if (score > bestScore) {
              bestScore = score;
              bestPage = p;
            }
          }
        }

        highlightedPage = bestPage;
        currentPage = bestPage;

        // --- Scroll to detected page inside the viewer-content div ---
        const targetPageEl = pdfCanvasContainer.querySelector(
          `[data-page-num="${bestPage}"]`
        ) as HTMLElement | null;

        if (targetPageEl && viewerContainer) {
          const containerTop = pdfCanvasContainer.offsetTop;
          const pageRelativeTop = targetPageEl.offsetTop;
          const scrollTarget = containerTop + pageRelativeTop - 20; // 20px breathing room

          viewerContainer.scrollTo({
            top: scrollTarget,
            behavior: 'smooth'
          });

          // Apply pulse glow class temporarily
          targetPageEl.classList.add('pulse-page-highlight');
          setTimeout(() => {
            targetPageEl.classList.remove('pulse-page-highlight');
          }, 2500);
        }
      } else if (viewerContainer) {
        // Fallback text view scroll
        const targetEl = viewerContainer.querySelector(`[data-violation-idx="${activeViolationIndex}"]`);
        if (targetEl) {
          targetEl.scrollIntoView({ behavior: 'smooth', block: 'center' });
        }
      }
    }
  });

  function changeZoom(delta: number) {
    soundState.playClick();
    const newZoom = Math.min(Math.max(0.6, zoomLevel + delta), 2.5);
    if (newZoom !== zoomLevel) {
      zoomLevel = newZoom;
      renderPdfPages();
    }
  }

</script>

<div class="doc-viewer-panel" class:fullscreen={isExpanded}>
  <!-- Document Viewer Header -->
  <div class="viewer-header">
    <div class="title-box">
      <span class="view-icon"><FileText size={15} /></span>
      <h3 class="doc-filename">{filename}</h3>
      <span class="char-length">{numPages} PAGES</span>
    </div>

    <div class="header-actions">
      <!-- PDF Attached Segmented Zoom Controls -->
      <div class="pdf-zoom-group">
        <button
          type="button"
          class="zoom-btn zoom-out"
          onclick={() => changeZoom(-0.15)}
          title={`Zoom Out (${Math.round(zoomLevel * 100)}%)`}
        >
          <Minus size={13} />
        </button>
        <button
          type="button"
          class="zoom-btn zoom-in"
          onclick={() => changeZoom(0.15)}
          title={`Zoom In (${Math.round(zoomLevel * 100)}%)`}
        >
          <Plus size={13} />
        </button>
      </div>

      <!-- Expand / Fullscreen Button at Top-Right Edge -->
      <button
        type="button"
        class="expand-btn"
        class:expanded={isExpanded}
        onclick={() => {
          soundState.playClick();
          isExpanded = !isExpanded;
        }}
        title={isExpanded ? 'Exit Fullscreen View (Esc)' : 'Expand PDF Card to Fullscreen'}
      >
        {#if isExpanded}
          <span class="btn-icon"><Minimize2 size={13} /></span> EXIT
        {:else}
          <span class="btn-icon"><Maximize2 size={13} /></span> EXPAND
        {/if}
      </button>
    </div>
  </div>

  <!-- PDF Canvas Viewer Surface -->
  <div class="viewer-content" bind:this={viewerContainer}>
    {#if isPdfLoading}
      <div class="pdf-loader">
        <div class="spinner"></div>
        <span>Rendering PDF pages...</span>
      </div>
    {:else if pdfError}
      <div class="pdf-error-box">
        <span class="err-icon"><AlertTriangle size={16} /></span>
        <span>{pdfError}</span>
      </div>
    {/if}

    <div class="pdf-canvas-container" bind:this={pdfCanvasContainer}></div>
  </div>
</div>

<style>
  .doc-viewer-panel {
    display: flex;
    flex-direction: column;
    height: 100%;
    min-height: 350px;
    background: #080f17;
    border: 1px solid rgba(107, 138, 153, 0.22);
    border-radius: 6px;
    overflow: hidden;
    box-shadow: 0 8px 30px rgba(0, 0, 0, 0.6);
    transition: all 0.35s cubic-bezier(0.4, 0, 0.2, 1);
  }

  .doc-viewer-panel.fullscreen {
    position: fixed;
    top: 0;
    left: 0;
    width: 100vw;
    height: 100vh;
    z-index: 999999;
    border-radius: 0;
    border: none;
    box-shadow: 0 0 60px rgba(0, 0, 0, 0.85);
  }

  .viewer-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 0.75rem 1.25rem;
    background: rgba(13, 23, 36, 0.95);
    border-bottom: 1px solid rgba(107, 138, 153, 0.2);
    flex-wrap: wrap;
    gap: 0.75rem;
  }

  .header-actions {
    display: flex;
    align-items: center;
    gap: 10px;
  }

  .expand-btn {
    display: flex;
    align-items: center;
    gap: 6px;
    font-family: var(--font-title);
    font-size: 0.72rem;
    font-weight: 700;
    letter-spacing: 1px;
    background: rgba(13, 23, 36, 0.95);
    border: 1px solid #132E35;
    color: #e2f1f8;
    padding: 0 12px;
    height: 28px;
    box-sizing: border-box;
    border-radius: 4px;
    cursor: pointer;
    transition: all 0.25s ease;
    box-shadow: 2px 2px 0 rgba(0, 0, 0, 0.25);
  }

  .expand-btn:hover {
    background: #1c3f48;
    border-color: #2a5863;
    color: #ffffff;
  }

  .expand-btn.expanded {
    background: #1c3f48;
    border-color: #2a5863;
    color: #ffffff;
  }

  .btn-icon {
    display: flex;
    align-items: center;
  }

  .title-box {
    display: flex;
    align-items: center;
    gap: 8px;
  }

  .view-icon {
    font-size: 1.1rem;
  }

  .doc-filename {
    font-family: var(--font-title);
    font-size: 0.9rem;
    font-weight: 700;
    color: #ffffff;
    letter-spacing: 0.5px;
    margin: 0;
  }

  .char-length {
    font-family: var(--font-mono);
    font-size: 0.65rem;
    color: var(--text-muted);
    background: rgba(255, 255, 255, 0.05);
    padding: 2px 6px;
    border-radius: 3px;
  }

  /* Attached Segmented Zoom Control Group */
  .pdf-zoom-group {
    display: inline-flex;
    align-items: center;
    height: 28px;
    box-shadow: 2px 2px 0 rgba(0, 0, 0, 0.25);
    border-radius: 4px;
  }

  .zoom-btn {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    width: 28px;
    height: 28px;
    padding: 0;
    background: rgba(13, 23, 36, 0.95);
    border: 1px solid #132E35;
    color: #e2f1f8;
    cursor: pointer;
    transition: all 0.2s cubic-bezier(0.16, 1, 0.3, 1);
    box-sizing: border-box;
  }

  /* Left button: outer-left rounded corners & inner right border */
  .zoom-btn.zoom-out {
    border-radius: 4px 0 0 4px;
    border-right: 1px solid rgba(19, 46, 53, 0.6);
  }

  /* Right button: outer-right rounded corners */
  .zoom-btn.zoom-in {
    border-radius: 0 4px 4px 0;
    border-left: none;
  }

  .zoom-btn:hover {
    background: #1c3f48;
    border-color: #2a5863;
    color: #ffffff;
  }

  .zoom-btn:active {
    transform: translateY(1px);
  }



  .viewer-content {
    flex: 1;
    overflow-y: auto;
    padding: 1.25rem;
    background: #060b12;
    display: flex;
    flex-direction: column;
    align-items: center;
    scrollbar-width: thin;
    scrollbar-color: rgba(107, 138, 153, 0.4) rgba(6, 11, 18, 0.9);
  }

  .viewer-content::-webkit-scrollbar {
    width: 6px;
  }

  .viewer-content::-webkit-scrollbar-track {
    background: rgba(6, 11, 18, 0.9);
  }

  .viewer-content::-webkit-scrollbar-thumb {
    background: rgba(107, 138, 153, 0.4);
    border-radius: 3px;
  }

  .viewer-content::-webkit-scrollbar-thumb:hover {
    background: var(--cyan-primary);
  }

  .pdf-loader {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 12px;
    padding: 3rem;
    color: var(--cyan-primary);
    font-family: var(--font-mono);
    font-size: 0.85rem;
  }

  .spinner {
    width: 28px;
    height: 28px;
    border: 3px solid rgba(107, 138, 153, 0.2);
    border-top-color: var(--cyan-primary);
    border-radius: 50%;
    animation: spin 0.8s linear infinite;
  }

  @keyframes spin {
    to {
      transform: rotate(360deg);
    }
  }

  .pdf-error-box {
    color: #ff2a70;
    font-family: var(--font-mono);
    font-size: 0.8rem;
    padding: 1rem;
    text-align: center;
  }

  .pdf-canvas-container {
    display: flex;
    flex-direction: column;
    gap: 1.5rem;
    width: 100%;
    align-items: center;
  }

  :global(.pdf-page-wrapper) {
    position: relative;
    box-shadow: 0 10px 30px rgba(0, 0, 0, 0.8);
    border-radius: 4px;
    overflow: hidden;
    transition: transform 0.3s ease, box-shadow 0.3s ease, outline 0.3s ease;
    background: #ffffff;
  }

  :global(.pdf-canvas) {
    display: block;
    max-width: 100%;
    height: auto;
  }

  :global(.pdf-page-number-badge) {
    position: absolute;
    bottom: 8px;
    right: 12px;
    background: rgba(0, 0, 0, 0.7);
    color: #ffffff;
    font-family: monospace;
    font-size: 0.65rem;
    padding: 2px 8px;
    border-radius: 3px;
    pointer-events: none;
  }

  :global(.pdf-page-wrapper.pulse-page-highlight) {
    outline: 3px solid var(--cyan-primary) !important;
    box-shadow: 0 0 25px rgba(107, 138, 153, 0.4) !important;
    animation: pdfPageGlow 0.8s ease-in-out infinite alternate !important;
  }

  @keyframes pdfPageGlow {
    0% {
      transform: scale(1);
    }
    100% {
      transform: scale(1.02);
    }
  }

  /* ─── Light Mode Overrides ─── */
  :global([data-theme="light"]) .doc-viewer-panel {
    background: #c2c6ca;
    border-color: #8a9aa8;
    box-shadow: 0 4px 20px rgba(0, 0, 0, 0.1);
  }

  :global([data-theme="light"]) .viewer-header {
    background: rgba(194, 198, 202, 0.95);
    border-bottom-color: #8a9aa8;
  }

  :global([data-theme="light"]) .view-icon {
    color: #69818d;
  }

  :global([data-theme="light"]) .doc-filename {
    color: #2d3a45;
  }

  :global([data-theme="light"]) .char-length {
    color: #5a6a78;
  }

  :global([data-theme="light"]) .pdf-zoom-group {
    box-shadow: 2px 2px 0 rgba(0, 0, 0, 0.1);
  }

  :global([data-theme="light"]) .zoom-btn {
    background: rgba(194, 198, 202, 0.95);
    border-color: #8a9aa8;
    color: #2d3a45;
  }

  :global([data-theme="light"]) .zoom-btn.zoom-out {
    border-right-color: rgba(138, 154, 168, 0.6);
  }

  :global([data-theme="light"]) .zoom-btn:hover {
    background: #69818d;
    border-color: #69818d;
    color: #ffffff;
  }

  :global([data-theme="light"]) .expand-btn {
    background: rgba(194, 198, 202, 0.95);
    border-color: #8a9aa8;
    color: #2d3a45;
    box-shadow: 2px 2px 0 rgba(0, 0, 0, 0.1);
  }

  :global([data-theme="light"]) .expand-btn:hover {
    background: #69818d;
    border-color: #69818d;
    color: #ffffff;
  }

  :global([data-theme="light"]) .expand-btn.expanded {
    background: #69818d;
    border-color: #8a9aa8;
    color: #ffffff;
  }

  :global([data-theme="light"]) .viewer-content {
    background: #b8bcc0;
  }

  /* ─── Mobile Responsiveness ─── */
  @media (max-width: 640px) {
    .viewer-header {
      padding: 0.5rem 0.75rem;
      gap: 0.5rem;
    }

    .title-box {
      max-width: 100%;
      overflow: hidden;
    }

    .doc-filename {
      font-size: 0.78rem;
      overflow: hidden;
      text-overflow: ellipsis;
      white-space: nowrap;
      max-width: 130px;
    }

    .char-length {
      font-size: 0.6rem;
      padding: 1px 4px;
    }

    .header-actions {
      gap: 6px;
    }

    .pdf-zoom-group {
      height: 24px;
    }

    .zoom-btn {
      width: 24px;
      height: 24px;
    }

    .expand-btn {
      padding: 0 8px;
      height: 24px;
      font-size: 0.62rem;
    }

    .viewer-content {
      padding: 0.4rem;
    }

    :global(.pdf-page-wrapper) {
      max-width: 100% !important;
      width: 100% !important;
    }

    :global(.pdf-canvas) {
      max-width: 100% !important;
      height: auto !important;
    }
  }
</style>
