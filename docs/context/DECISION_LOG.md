# Architectural And Workflow Decision Log

Context role: Canonical consequential-decision ledger.
Scope: Product architecture, evidence, context, packet transport, and integration workflow.
Canonical for: Decision status, reasons, consequences, and reversal conditions.
Not canonical for: Current branch state or detailed subsystem implementation.
Inherits: [`README.md`](README.md), root law, and [`CURRENT_STATE.md`](CURRENT_STATE.md).
Current-state owner: [`CURRENT_STATE.md`](CURRENT_STATE.md).
Related evidence: Source, Git history, subsystem pages, and proof records.
Update triggers: A decision is accepted, superseded, or meets its reversal condition.

| Decision | Evidence | Reason / rejected alternative | Consequence and law owner | Reversal condition | Status |
| --- | --- | --- | --- | --- | --- |
| Plain terminal output is the baseline product oracle. | `USER_DECISION`, `LIVE_REPO_FACT` | Reject UI summaries as stronger truth. | CLI tests and proof own exact transcripts; root AGENTS owns law. | Don explicitly changes product surface with replacement proof. | Current |
| Exact removes at most one LF or CRLF and changes nothing else. | `USER_DECISION`, `LIVE_REPO_FACT` | Reject fuzzy, formatting, or classifier-based passing. | `answer` owns Exact; validators cannot weaken it. | Explicit product-law decision plus rewritten tests. | Current |
| Module authority remains separated. | `LIVE_REPO_FACT` | Reject a central god module. | Matrix and subsystem pages record catalog, engine, client, adapter, and tool boundaries. | Source earns a different stable owner. | Current |
| Client is terminal-free; TUI is a projection. | `USER_DECISION`, `LIVE_REPO_FACT` | Reject Ratatui/Crossterm model types in client truth. | Client owns portable state; TUI owns events/render/lifecycle. | Playtesting proves a concrete defect requiring redesign. | Current, sealed |
| RustfmtExact and tool runner stay inert in normal product flow. | `LIVE_REPO_FACT` | Reject early non-exact validation and arbitrary command service. | Validators fail closed; real runner accepts fixed rustfmt requests only. | Separate authorized integration packet with failure proof. | Current |
| Procgen uses explicit static source and checked deterministic expansion. | `USER_DECISION`, `LIVE_REPO_FACT` | Reject PRNG/hash order, runtime LLM, opaque templates, and engine generation. | Procgen output is inert until catalog admission. | Demonstrated finite-domain limitation with accepted replacement design. | Current |
| Engine never generates curriculum. | `USER_DECISION`, `LIVE_REPO_FACT` | Reject generator logic in session truth. | Catalog-admitted ordinary reps alone feed engine. | No planned reversal. | Current |
| Packets are complete manual transport objects. | `USER_DECISION` | Reject fragments, implicit values, and Don filling routine mechanics. | Packet protocol requires envelope, causality, literal fields, and fragment rejection. | A proven transport replaces manual packets without losing semantics. | Current |
| Supervisors own lane meaning; Master alone integrates shared truth. | `USER_DECISION` | Reject worker self-integration or inferred private chat state. | Workers expose live evidence; Master accepts/merges; no automatic pushes. | Explicit governance change. | Current |
| Context is recursive and evidence-labeled. | `USER_DECISION` | Reject chat-only durable memory and copied mutable truth. | Root-to-local AGENTS, canonical owners, and Context-SHA govern discovery. | Context system demonstrably increases contradiction or maintenance cost. | Current |
| Consequential packets cannot be silent fragments. | `USER_DECISION` | Reject truncated or separately interpreted semantic reports. | Outbox packets are immutable and structurally verified. | Transport supports atomic equivalent envelopes. | Current |
| Runtime LLM curriculum generation is excluded. | `USER_DECISION` | Reject nondeterministic, unauditable generated product content. | Generation must remain local, finite, deterministic, and inspectable. | Explicit Don decision with reproducibility law preserved. | Current |
| Feature progression pauses for infrastructure. | `USER_DECISION` | Reject parallel feature drift while context owners are established. | Materialization, multilingual guidance, selector, curriculum, validators, and lab changes remain paused. | Master accepts infrastructure and issues context deltas. | Current |
| Fleet tools remain standalone, std-only, and non-destructive. | `USER_DECISION` | Reject product dependency, daemon, dashboard, or mutation framework. | Tools compile outside product Cargo and default to diagnostics. | A later tooling need is demonstrated and separately authorized. | Current |
