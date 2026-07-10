# Deterministic Procgen

Context role: Subsystem authority and navigation.
Scope: `src/curriculum/procgen.rs`, `src/curriculum/procgen/expansion.rs`, and the adjacent static source boundary in `src/curriculum/guidance.rs`.
Canonical for: Locale-independent task source, pure expansion, generated provenance, static guidance ownership, and isolation law.
Not canonical for: Catalog admission, playable identity, session behavior, or learning scheduling.
Inherits: [`../../../AGENTS.md`](../../../AGENTS.md) and curriculum scoped law.
Current-state owner: [`../CURRENT_STATE.md`](../CURRENT_STATE.md).
Related evidence: Procgen unit tests and the accepted `/tmp` architecture artifact.
Update triggers: Template/domain schema, expansion contract, provenance, or admission state changes.

## Mission And Authority

Procgen describes finite ordered symbolic source and expands a template plus
ordered domains plus a `u64` content seed into an owned inert `ExpandedTask`.
Allowed authority is semantic/surface source, structured `DomainValueSpec`
values, optional meaning keys, explicit literal/slot parts, checked mixed-radix
selection, exact stimulus and expected rendering, copied metadata, explicit
errors, and inert generated identity.

Forbidden authority includes catalog short IDs, playable canonical IDs,
registration, engine packets, runtime LLM generation, randomization, HashMap
iteration, persistence, scheduling, and answer checking.

The ownership pipeline is:

```text
static TemplateSpec + ordered SlotDomain source + content_seed
  -> owned ExpandedTask
  -> future catalog admission
  -> future guidance resolution
  -> future fully resolved ExercisePacket
  -> engine session truth
```

Only the first arrow exists. Source and expansion may depend on curriculum
metadata enums; no product module may call expansion before admission.

`TemplateSpec` separates `instruction_key`, `stimulus`, and `expected`.
`ExpandedTask` copies the instruction key and uses `content_seed` and
`stimulus_variant`; selected assignments preserve optional meaning keys. The
generated identity remains locale-independent and the engine never re-expands a
template.

## Adjacent Static Guidance Source

`src/curriculum/guidance.rs` owns static reviewed instruction source only. The
accepted source contains one `en@v1` pack with instruction key
`copy.exact_line`, exact literal `Copy this line exactly:`, and no glosses.

Guidance may describe locale, pack version, instruction parts, and optional
glosses. It may not own or mutate expected answers, validate attempts, admit
catalog content, create engine truth, perform client/TUI lookup, translate at
runtime, or choose a fallback. No resolver or product caller exists.

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

Pure Guidance Resolution V1 is planned but not active. Materialization remains
paused. Compatibility rows for non-Cartesian systems, presentation provenance,
fallback behavior, and all catalog admission semantics remain deferred. Update
on any task/guidance source, expansion, resolution, versioning, determinism,
identity, isolation, or admission change.
