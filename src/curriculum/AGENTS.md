# AGENTS: Curriculum And Procgen

## Scope And Inheritance

Applies under `src/curriculum/`. Inherits [`../../AGENTS.md`](../../AGENTS.md).

## Owner And Authority

Curriculum owns static schema/data. Procgen owns inert source and pure expansion.
Catalog admission, short IDs, ordering, and playability remain catalog authority.
Engine/session truth, terminal output, runtime generation, persistence, and
randomness are forbidden here.

Allowed dependencies point inward to curriculum enums/data and standard library
only. Product modules must not call procgen before an explicit catalog-admission
packet. Procgen must not import catalog, engine, client, TUI, practice, or lab.

## Canonical Links

- Root law: [`../../AGENTS.md`](../../AGENTS.md)
- Index: [`../../docs/INDEX.md`](../../docs/INDEX.md)
- Current state: [`../../docs/context/CURRENT_STATE.md`](../../docs/context/CURRENT_STATE.md)
- Curriculum/catalog: [`../../docs/context/subsystems/curriculum-catalog.md`](../../docs/context/subsystems/curriculum-catalog.md)
- Procgen: [`../../docs/context/subsystems/procgen.md`](../../docs/context/subsystems/procgen.md)
- Lane registry: [`../../docs/lanes/lane-registry.md`](../../docs/lanes/lane-registry.md)

## Validation And Stop Conditions

Run module-local invariants, procgen deterministic/isolation tests when touched,
catalog tests when admitted data changes, and full project proof. Stop if IDs,
Exact behavior, catalog listing, or product callers change outside explicit
authority. Update on schema, determinism, or admission-boundary changes.
