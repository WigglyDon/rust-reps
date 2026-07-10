# Project History

Context role: Canonical direction-change history.
Scope: Major product and architecture eras evidenced by Git, source, and retained audits.
Canonical for: Era transitions, superseded directions, donors, and remaining debt.
Not canonical for: Current state or exhaustive commit chronology.
Inherits: [`README.md`](README.md) and [`CURRENT_STATE.md`](CURRENT_STATE.md).
Current-state owner: [`CURRENT_STATE.md`](CURRENT_STATE.md).
Related evidence: Git commits, retained audits, source, and packet proof artifacts.
Update triggers: A major direction changes, is retired, or gains stronger evidence.

| Era / status | Evidence class | Prior direction and trigger | Revised direction | Invariant law | Remaining debt / evidence |
| --- | --- | --- | --- | --- | --- |
| Exact CLI foundation / current base law | `LIVE_REPO_FACT` | Initial V0 (`b98c8aa`) grew practice, commands, proof, and project rename through early commits. | Plain stdout/stderr became the baseline oracle with wrong answers as results. | Exact trims at most one LF/CRLF; no other normalization. | Terminal contracts remain transcript-heavy by design. |
| TUI adoption and client seam / current, sealed | `LIVE_REPO_FACT` | CLI-only interaction gained Ratatui/Crossterm (`789537c`); repeated UI refinement exposed ownership risk. | Engine packet, terminal-free client model, and TUI projection were separated; boundary sealed at `5e924d8`. | TUI is one client; engine owns truth. | Real-terminal feel remains manual evidence. |
| Curriculum schema and catalog metadata / current | `LIVE_REPO_FACT` | Flat drill data gained schema/sharding (`c583429`, `c3ac7f7`) and metadata (`ee24495`). | Catalog owns IDs, metadata, ordering, lookup, and invariants. | Data feeds engine; engine does not author curriculum. | Future generated admission is not designed in product code. |
| Validator/tool capability seams / current but inert | `LIVE_REPO_FACT` | Exact-only dispatch gained explicit future capability markers (`563e268` through `0c3b5d7`). | RustfmtExact core and fixed runner exist but normal product dispatch remains closed. | Non-exact classification never makes an Exact mismatch pass. | Product integration is deliberately unauthorized. |
| Language-agnostic procgen design / current architecture | `WORKER_CLAIM` revalidated where source exists | `/tmp` architecture design selected semantic/surface separation, operations, explicit parts, and mixed-radix expansion. | Static source (`9946391`) and pure owned expansion (`5f8a1d9`) implement only the inert first arrow. | Deterministic source; owned output; catalog admission later; no runtime LLM. | Materialization, compatibility rows, admission, and non-Copy product prompts remain future. |
| Multilingual guidance/selector / planned next, unstarted in repo | `USER_DECISION` plus `UNKNOWN` | Packet intent exists, but discovery found no branch, worktree, or artifact establishing live implementation. | Cross-Lingual Guidance and Presentation Architecture V1 is the next planned design-only boundary; do not invent a worker. | New work begins only through a fresh lane packet and current Context-SHA. | Design, selector, implementation, and product authority remain unstarted. |
| Recursive context and fleet infrastructure / current accepted infrastructure | `LIVE_REPO_FACT` | Durable law was concentrated in root AGENTS, source, Git, `/tmp` artifacts, and manually transported packets. | Canonical context owners and standalone diagnostics were accepted through candidate lineage `499e36c`, `bd06952`, `b4295fa`, `b7a91f4`, and `34874d51a6ec54a5ec219f9f2ca99d1449eadf6e`. | Source/runtime determine behavior; packets carry tasks; context carries memory. | Central release-gate wiring remains deferred; semantic contradictions still require Master judgment. |

The accepted infrastructure lineage is
`499e36cc51543dfe766df3556b2079faeacbe3d7`,
`bd069520aa19086e637c5ec895989c856b372db2`,
`b4295fad11e00cdbfc034356e5a5a215e33508b1`,
`b7a91f42d7d0d9df0a2a2feeef38d883a9783d3b`, and
`34874d51a6ec54a5ec219f9f2ca99d1449eadf6e` in that order.

## Superseded And Donor Paths

- `LIVE_REPO_FACT`: `docs/codex_tui_reference_audit.md` is a useful donor for
  TUI lifecycle ideas, but its proposed module plan and next-packet wording are
  superseded by the live split under `src/tui` and `src/client`.
- `LIVE_REPO_FACT`: `docs/editor_backend_audit.md` recommended future
  Tree-sitter dependencies, but live source instead earned a first-party editor
  without those dependencies. The dependency proposal is historical, not current.
- `LIVE_REPO_FACT`: `curriculum-sharding-v0` is an ancestor of master and a
  historical milestone. `curriculum-data-seam-v0` diverges from an old base and
  is preserved as a parked donor, not an active lane.
- `UNKNOWN`: remote history and remote tracking cannot be reconstructed because
  no remote is configured.
