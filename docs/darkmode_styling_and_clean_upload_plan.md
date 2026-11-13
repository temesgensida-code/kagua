# Implementation Plan: Muted Dark Mode Palette & Clean Document Upload UI

**Target Goal**:
1. Eliminate all beaming neon light-blue glows, electric text-shadows, and bright cyan borders across dark mode, replacing them with an understated, low-lightness Steel Slate & Muted Greyish-Blue palette (`#6b8a99` / `#4a6775` / `#253842`).
2. Remove the horizontal laser scan line and technical console logs during document upload, replacing them with a simple circular ring spinner and clean, human-friendly status messages: `"working..."` &rarr; `"on the way..."` &rarr; `"ready"`.

---

## 1. Confirmed Design Decisions (from `/grill-me` Review)

| # | Architecture Decision | Confirmed Approach | Rationale |
|---|---|---|---|
| **1** | **Upload Progress Mapping** | **Lifecycle Stage Mapping** | Automatically map internal pipeline events to friendly copy: `UPLOAD_RECEIVED` & `PARSING_DOCUMENT` &rarr; `"working..."`; `FACTS_EXTRACTED` & `REASONING_PROLOG` &rarr; `"on the way..."`; completion &rarr; `"ready"`. |
| **2** | **Dark Mode Color Palette** | **Steel Slate & Muted Greyish-Blue (`#6b8a99` / `#4a6775`)** | Reduces lightness, adds subtle greyishness, and removes all electric beam shadows (`text-shadow: none`, `box-shadow: none` / dark neutral shadows). |
| **3** | **Upload HUD Visuals** | **Clean Circular Ring Spinner** | Smooth 36px ring with a slate track and muted slate-blue head spinning gently above the single status message; zero technical logs, zero laser bar. |
| **4** | **Scope of Execution** | **Comprehensive Application-Wide Cleanup** | Applied globally in `app.css` and consistently across `DropZone`, `HeaderSection`, `AnalysisResultView`, `DocumentViewer`, and `Auth` cards. |

---

## 2. Palette Transformation Specification

| Token / Usage | Old Electric Value | New Muted Steel Slate Value | Effect |
|---|---|---|---|
| `--cyan-primary` | `#00f0ff` (high-lightness neon cyan) | `#6b8a99` (muted slate blue) | Eliminates bright electric beam; softer on the eyes. |
| `--cyan-glow` | `rgba(0, 240, 255, 0.35)` | `rgba(107, 138, 153, 0.15)` | Replaces neon glow with subtle ambient depth. |
| `--cyan-muted` | `#0090a8` | `#4a6775` | Darker, calm steel accent for hover states. |
| Border Accents | `rgba(0, 240, 255, 0.18)` | `rgba(107, 138, 153, 0.22)` or `#1e353d` | Clean, crisp, non-distracting panel outlines. |
| Text Shadows | `text-shadow: 0 0 10px rgba(0, 240, 255, 0.4)` | `text-shadow: none` | Removes blurry glowing halo around header & labels. |
| Active Card Glows | `box-shadow: 0 0 14px rgba(0, 240, 255, 0.25)` | `box-shadow: 0 4px 12px rgba(0, 0, 0, 0.35)` | Replaces neon beam with refined dark drop-shadow. |
| Upload Laser Bar | `box-shadow: 0 0 15px #00f0ff, 0 0 30px #00f0ff` | **Removed completely** | No horizontal scanning laser bar. |

---

## 3. Step-by-Step Implementation Specifications

### Phase 1: Global CSS Variable Cleanup (`ui/src/app.css`)
1. Update root CSS variables:
   ```css
   :root {
     /* Replace electric cyan with muted steel slate */
     --cyan-primary: #6b8a99;
     --cyan-glow: rgba(107, 138, 153, 0.15);
     --cyan-muted: #4a6775;
     --grid-color: rgba(107, 138, 153, 0.03);
   }
   ```
2. Remove glowing keyframe box-shadows:
   - In `.cyber-corner`, `.cyber-border-box`, and `.pulse-glow`, remove high-intensity `0 0 25px rgba(0, 240, 255, ...)` beams, replacing with subtle `rgba(0, 0, 0, 0.4)` shadows or clean border highlights.

---

### Phase 2: Simplified Upload HUD in `DropZone.svelte`

#### 2.1 Remove Laser Bar & Technical Logs
1. Delete the `<div class="scan-laser"></div>` element and its `@keyframes scanAnimation`.
2. Delete the `<div class="progress-log">` section that rendered raw `[UPLOAD_RECEIVED]`, `[PARSING_DOCUMENT]`, `[REASONING_PROLOG]` logs.

#### 2.2 Modernized Minimalist Spinner & Friendly Messages
Update the scanning overlay template in `DropZone.svelte`:
```svelte
{#if isScanning}
  <div class="scanning-hud">
    <div class="spinner"></div>
    <span class="scan-stage">{friendlyStageMessage}</span>
  </div>
{/if}
```

#### 2.3 Friendly Message Mapping
In `DropZone.svelte` (or computed in `+page.svelte`):
```typescript
let friendlyStageMessage = $derived.by(() => {
  if (!currentStageMessage) return "working...";
  const msgUpper = currentStageMessage.toUpperCase();

  if (msgUpper.includes("COMPLETED") || msgUpper.includes("READY")) {
    return "ready";
  }
  if (
    msgUpper.includes("PROLOG") ||
    msgUpper.includes("REASONING") ||
    msgUpper.includes("OFFSET") ||
    msgUpper.includes("MAPPING") ||
    msgUpper.includes("FACTS")
  ) {
    return "on the way...";
  }
  return "working...";
});
```

#### 2.4 Spinner & HUD Styling
```css
.scanning-hud {
  position: absolute;
  inset: 0;
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  background: rgba(8, 15, 24, 0.92);
  backdrop-filter: blur(4px);
  z-index: 10;
  gap: 1rem;
}

.spinner {
  width: 36px;
  height: 36px;
  border: 3px solid rgba(107, 138, 153, 0.2);
  border-top-color: #6b8a99;
  border-radius: 50%;
  animation: spin 0.9s cubic-bezier(0.5, 0.1, 0.4, 0.9) infinite;
}

.scan-stage {
  font-family: var(--font-mono);
  font-size: 0.85rem;
  font-weight: 600;
  color: #a0b3c2;
  letter-spacing: 1.5px;
  text-transform: lowercase;
}
```

#### 2.5 Document Icon Color Update
In `DropZone.svelte`, update the upload SVG path strokes from `#00f0ff` to `#6b8a99`.

---

### Phase 3: Header & Navigation Glow Removal (`HeaderSection.svelte`)

1. Remove text shadows from the main logo and subheaders:
   ```css
   /* Before: text-shadow: 0 0 10px rgba(0, 240, 255, 0.4); */
   /* After: */
   text-shadow: none;
   ```
2. Replace borders:
   - Change `border: 1px solid rgba(0, 240, 255, 0.15)` to `border: 1px solid rgba(107, 138, 153, 0.2)`.

---

### Phase 4: Analysis Results & Findings Cleanup (`AnalysisResultView.svelte`)

1. **Card Border & Background Highlights**:
   - Change `rgba(0, 240, 255, ...)` borders on `.action-card`, `.findings-card`, and `.provenance-card` to `rgba(107, 138, 153, 0.2)`.
2. **Active Finding Selection**:
   - Replace neon cyan focus glow `box-shadow: 0 0 14px rgba(0, 240, 255, 0.25)` with a clean, solid border highlight:
     ```css
     .finding-item.selected {
       border-color: #6b8a99;
       box-shadow: 0 4px 12px rgba(0, 0, 0, 0.35);
       background: rgba(107, 138, 153, 0.06);
     }
     ```
3. **Offset and Detail Badges**:
   - Update `.offset-badge` and code tags from bright electric blue to muted slate `#6b8a99`.

---

### Phase 5: Document Viewer Styling (`DocumentViewer.svelte`)

1. Update `.doc-viewer-panel`:
   - Change `border: 1px solid rgba(0, 240, 255, 0.18)` to `border: 1px solid #1a2a35`.
2. Remove fullscreen glowing beam:
   - Change `box-shadow: 0 0 60px rgba(0, 240, 255, 0.35)` to clean dark backdrop shadow `box-shadow: 0 20px 60px rgba(0, 0, 0, 0.8)`.
3. Update `.pulse-page-highlight`:
   - Replace the high-intensity neon cyan box-shadow (`box-shadow: 0 0 35px var(--cyan-primary), 0 0 60px rgba(0, 240, 255, 0.7)`) with a gentle, non-blinding highlight (`box-shadow: 0 0 16px rgba(107, 138, 153, 0.45)`).

---

### Phase 6: Auth Pages & Modal Cleanup

1. Remove electric `text-shadow: 0 0 30px rgba(0, 240, 255, 0.4)` from page titles in `login`, `register`, `forgot-password`, and `verify-email`.
2. Update `AuthButton.svelte` and `AuthInput.svelte`:
   - Replace `#00f0ff` button gradients and glowing focus rings with clean slate buttons and `#6b8a99` focus outlines.

---

## 4. Verification & Testing Checklist

| Test Case | Interaction / Screen | Expected Behavior |
|---|---|---|
| **TC-UI-01: File Upload Transition** | Upload any document in DropZone. | No horizontal scanning laser bar appears; smooth circular spinner renders; text displays `"working..."` &rarr; `"on the way..."` &rarr; `"ready"`. |
| **TC-UI-02: Zero Technical Jargon** | Observe upload HUD. | No `[PARSING_DOCUMENT]`, `[UPLOAD_RECEIVED]`, or backend execution logs displayed to the user. |
| **TC-UI-03: Dark Mode Edge Beams** | Inspect panels, cards, and dropzone in dark mode. | Zero electric glowing borders; edges are muted slate/grey (`#253842` / `rgba(107, 138, 153, 0.22)`). |
| **TC-UI-04: Header Text Shadows** | Inspect KAGUA header, subtitles, and badges. | No glowing text halo / beam; typography is crisp, clean, and legible. |
| **TC-UI-05: Selected Card Highlight** | Click a finding card in AnalysisResultView. | Selected card shows a clean slate border and dark shadow, without neon cyan bloom. |
| **TC-UI-06: Document Viewer Fullscreen** | Click EXPAND on Document Viewer. | Panel expands fullscreen with a clean dark backdrop shadow, no cyan edge glow. |
