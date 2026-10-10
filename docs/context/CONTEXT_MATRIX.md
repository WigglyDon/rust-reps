# Architecture and ownership

| Owner | Responsibility and dependency direction |
| --- | --- |
| `curriculum` | Static task/schema data, explicit answer/format properties, support metadata and authored guidance. No session or terminal ownership. |
| `catalog` | Admission, ordering, IDs, lookup and invariants over curriculum data. Does not run tools or expand templates at runtime. |
| `curriculum::procgen` | Pure expansion and reviewed static materialization. Runtime product does not expand templates. |
| `curriculum::planner` | Selects caller-supplied admitted candidates under explicit capability/guidance/rule constraints. Does not inspect answer bytes or own catalog lookup. |
| `curriculum::guidance` | Authored prose and explicit locale/key resolution. Does not own stimuli, expected answers, validation or learner results. |
| `orchestration` | Catalog lookup, guidance/support resolution, admitted-ID planning and resolved packet construction. Supplies the engine; adapters do not duplicate this work. |
| `engine` | Pure session state, attempts, pass/wrong/skip/completion and private expected-answer validation. No catalog/guidance lookup or terminal types. |
| `answer`, `validators`, `tool_runner`, `formatter` | Literal comparison, explicit rule dispatch, fixed rustfmt execution and separate editor formatting. No arbitrary learner commands or semantic-equivalence claim. |
| `main`, `cli`, `practice`, `presentation` | Launch selection and deterministic plain terminal adaptation using resolved packets and engine results. Wrong answers remain normal results; tool errors are explicit. |
| `client` | Portable editor, cursor/viewports, visual feedback and projection-safe task/help state. No terminal library types or answer/session authority. |
| `workshop` | Persistent learner source, revision/build identity, eligible artifact and separate compiler/program records. Uses the process adapter; separate from course engine and validators. |
| `workshop_process` | Fixed single-file rustc and explicit artifact launch, minimal child environment, bounded capture/time and Unix group cleanup. No shell or security-sandbox claim. |
| `curriculum::workshop`, `client::workshop`, `tui::workshop` | Respectively preview teaching prose, portable editor/focus/scroll projection and terminal events/rendering. Consume shared workshop truth; never insert solutions on step changes. |
| `tui` | Terminal lifecycle, input mapping, frame scheduling, drawing and restoration. Uses shared client/engine semantics. |
| `lab` | Explicit synthetic stress packets, separate from the playable catalog and curriculum. |
| `tools/fleet`, `xtask` | Developer checks, local observations, deterministic context/evidence generation and optional historical diagnostics. Never a product dependency or authority source. |

Mutable status belongs to [current state](CURRENT_STATE.md); source establishes
implementation. [The subsystem pages](subsystems/) hold relevant detailed
contracts. [Development](../DEVELOPMENT.md) owns task, Git and resource workflow.
