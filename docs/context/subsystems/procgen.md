# Deterministic Procgen

Context role: Subsystem authority and navigation.
Scope: `src/curriculum/procgen.rs` and `src/curriculum/procgen/expansion.rs`.
Canonical for: Static source, pure expansion, generated provenance, and isolation law.
Not canonical for: Catalog admission, playable identity, session behavior, or learning scheduling.
Inherits: [`../../../AGENTS.md`](../../../AGENTS.md) and curriculum scoped law.
Current-state owner: [`../CURRENT_STATE.md`](../CURRENT_STATE.md).
Related evidence: Procgen unit tests and the accepted `/tmp` architecture artifact.
Update triggers: Template/domain schema, expansion contract, provenance, or admission state changes.

## Mission And Authority

Procgen describes finite ordered symbolic source and expands a template plus
ordered domains plus a `u64` seed into owned inert output. Allowed authority is
semantic/surface source, explicit literal/slot parts, checked mixed-radix
selection, exact rendering, copied metadata, explicit errors, and inert
generated identity.

Forbidden authority includes catalog short IDs, playable canonical IDs,
registration, engine packets, runtime LLM generation, randomization, HashMap
iteration, persistence, scheduling, and answer checking.

The ownership pipeline is:

```text
static TemplateSpec + ordered SlotDomain source + seed
  -> owned ExpandedRep
  -> future catalog admission
  -> future ordinary RepSpec
  -> future engine ExercisePacket
```

Only the first arrow exists. Source and expansion may depend on curriculum
metadata enums; no product module may call expansion before admission.

## Determinism And Proof

Slot axes follow template order; the first slot varies fastest. Cardinality and
conversions are checked; valid seeds are exactly `0..cardinality`. Rendering
appends literal and selected slot bytes without normalization. Metadata order is
copied, not merged or sorted. Generated IDs are inert provenance.

Accepted proof: exhaustive current range, mixed-radix, zero-slot, renderer byte,
failure, identity, reproducibility, and duplicate-signature tests plus source
isolation audits. These do not prove curriculum quality, Unicode composition in
general, catalog admission, or product prompts.

Rollback is removal of an unadmitted candidate without catalog impact.
Observability is deterministic test output; runtime/product observability is
absent. Resource bounds are finite domain cardinality and owned strings/vectors.

## Gate And Unknowns

Materialization is paused. Compatibility rows for non-Cartesian systems and all
catalog admission semantics remain deferred. Update on any source/expansion
versioning, determinism, identity, isolation, or admission change.
