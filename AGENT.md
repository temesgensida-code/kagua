# Project Development & Autonomous Agent Orchestration Rules

## 1. Master Orchestrator Directive (`docs/overview.md`)

All engineering agents operating in this repository **MUST** follow [`docs/overview.md`](file:///home/temesgensida/tom/prolog/Kagua/docs/overview.md) as the primary orchestration authority. 

Before modifying or executing any code:
1. **Consult the Overview**: Inspect [`docs/overview.md`](file:///home/temesgensida/tom/prolog/Kagua/docs/overview.md) to understand where the task sits in the global dependency graph.
2. **Verify Wave Prerequisites**: Ensure that all antecedent waves and prerequisite plans are implemented and verified. Never execute tasks out of sequence.
3. **Respect the 4-Wave Migration Protocol**:
   - **Wave 1: Core Foundation & Concurrency Safety** ([`docs/production_reliability_plan.md`](file:///home/temesgensida/tom/prolog/Kagua/docs/production_reliability_plan.md))
     *Prolog thread_local isolation, Postgres exponential retry loop, Axum 25MB body limit, shared Reqwest pool with cold-start backoff, 150 DPI memory-capped OCR.*
   - **Wave 2: Accurate Extraction & Deterministic Navigation** ([`docs/no_ai_accuracy_improvement_plan.md`](file:///home/temesgensida/tom/prolog/Kagua/docs/no_ai_accuracy_improvement_plan.md) & [`docs/pdf_highlight_navigation_plan.md`](file:///home/temesgensida/tom/prolog/Kagua/docs/pdf_highlight_navigation_plan.md))
     *Clause-scoped extraction, unit-bound regexes, polarity scoper, statutory bounds, page boundary tracking, offset-to-page resolution, omission badging, and PDF.js TextLayer.*
   - **Wave 3: Core Workspace Layout** ([`docs/horizontal_resizable_split_plan.md`](file:///home/temesgensida/tom/prolog/Kagua/docs/horizontal_resizable_split_plan.md))
     *Continuous draggable splitter (25%–75%), quick-snap chevrons, and localStorage ratio persistence in `+page.svelte`.*
   - **Wave 4: UI Controls, Clean Upload & Visual Polish** ([`docs/darkmode_styling_and_clean_upload_plan.md`](file:///home/temesgensida/tom/prolog/Kagua/docs/darkmode_styling_and_clean_upload_plan.md), [`docs/export_buttons_cleanup_and_layout_plan.md`](file:///home/temesgensida/tom/prolog/Kagua/docs/export_buttons_cleanup_and_layout_plan.md), [`docs/pdf_zoom_buttons_refinement_plan.md`](file:///home/temesgensida/tom/prolog/Kagua/docs/pdf_zoom_buttons_refinement_plan.md))
     *Steel slate dark mode (`#6b8a99`), clean circular spinner HUD with status copy, removal of CSV/JSON dead code with 50/50 flex grid, and unified `[-][+]` zoom pill.*

---

## 2. File Collision & Conflict Prevention Rules

High-collision files are touched by multiple plans. Agents modifying these files must strictly check and preserve cross-plan features:

1. **`parser/services/extractor.py`**:
   - Preserves 150 DPI downscaling, max 35 page cap, and iterative garbage collection (PLAN-01).
   - Preserves page boundary indexing `pages_meta` (PLAN-03).
2. **`core-be/src/service.rs`**:
   - Preserves singleton `reqwest::Client` connection pool and cold-start backoff retry loop (PLAN-01).
   - Preserves offset-to-page resolution and omission badge mapping (PLAN-03).
3. **`ui/src/routes/(app)/+page.svelte`**:
   - Preserves the horizontal split-screen container, drag event listeners, and snap actions (PLAN-04).
   - Preserves the clean circular upload HUD and status copy mapping (PLAN-05).
4. **`ui/src/lib/components/DocumentViewer.svelte`**:
   - Preserves the PDF.js TextLayer highlight overlays and deterministic scrolling (PLAN-03).
   - Preserves the fused `[-][+]` segmented zoom pill controls and borders (PLAN-07).
5. **`ui/src/lib/components/AnalysisResultView.svelte`**:
   - Preserves omission findings badges and page jump actions (PLAN-03).
   - Preserves the 50/50 flex-wrap grid for PDF/Markdown export buttons (PLAN-06).
   - Preserves the muted steel slate color palette (PLAN-05).
6. **`reasoner-engine/engine.pl`**:
   - Preserves `:- thread_local user:doc_fact/2.` and transactional `setup_call_cleanup/3` scoping (PLAN-01).

---

## 3. General Engineering Rules

- Read the existing implementation before changing it.
- Follow the architecture already established in the project.
- Do not rewrite working code without a clear reason.
- Prefer small, focused changes.
- Avoid unrelated refactoring.

---

## 4. Implementation & Progression Rules

- Read the relevant implementation plan in `docs/` before starting work.
- Never silently skip a requirement.
- Update the relevant plan and `docs/overview.md` when a task or milestone is completed.
- Commit / version control changes when each milestone is complete.
- **Approval Gate**: Ask the user's permission before moving from one milestone / wave to the next milestone / wave.

---

## 5. Testing & Verification Gates

At the end of each wave or milestone, execute the relevant verification commands:
- **Rust Core Backend**: Run `cargo check` and `cargo test` in `core-be/`.
- **SvelteKit Frontend**: Run `pnpm --filter ui check` in `ui/`.
- **Python Parser**: Run `python -m py_compile` across `parser/services/*.py`.
- **SWI-Prolog Reasoner**: Verify syntax and module loading via `swipl -g "use_module(engine), halt." reasoner-engine/engine.pl`.

Never claim a test passed without actually running it and observing the clean exit code.

---

## 6. Safety

- Never expose secrets, API keys, or credentials.
- Never delete user data or audit reports without authorization.
- Never make destructive database changes or migrations without approval.
- Preserve unrelated modifications in the working tree.

---

## 7. Completion Checklist

Before declaring any implementation plan or wave complete:
1. Inspect the repository's git status and diffs (`git diff`).
2. Run automated test suites and linters.
3. Verify against the acceptance criteria defined in the specific implementation plan in `docs/`.
4. Check for regressions in overlapping files listed in `docs/overview.md`.
5. Report findings clearly, distinguishing between:
   - **Implemented and verified**
   - **Implemented but pending verification**
   - **Not implemented**
   - **Blocked by an external dependency or decision**
