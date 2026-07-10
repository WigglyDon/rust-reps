# Deterministic Procgen

Context role: Subsystem authority and navigation.
Scope: `src/curriculum/procgen.rs`, `src/curriculum/procgen/expansion.rs`, `src/curriculum/guidance.rs`, and `src/curriculum/guidance/resolution.rs`.
Canonical for: Locale-independent task source, pure expansion, generated provenance, static guidance ownership, pure guidance resolution, and isolation law.
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
static GuidancePack + locale + pack version + owned ExpandedTask
  -> inert ResolvedGuidance
future catalog admission + owned ExpandedTask + ResolvedGuidance
  -> future fully resolved ExercisePacket
  -> engine session truth
```

The locale-independent expansion and inert guidance-resolution arrows exist.
Catalog admission, orchestration, packet construction, and every product arrow
remain absent. Source and expansion may depend on curriculum metadata enums; no
product module may call expansion or guidance resolution before admission.

`TemplateSpec` separates `instruction_key`, `stimulus`, and `expected`.
`ExpandedTask` copies the instruction key and uses `content_seed` and
`stimulus_variant`; selected assignments preserve optional meaning keys. The
generated identity remains locale-independent and the engine never re-expands a
template.

## Adjacent Static Guidance Source

`src/curriculum/guidance.rs` owns static reviewed instruction source only. The
accepted inert source contains `en@v1` and proof-only `uk@v1`, both with
instruction key `copy.exact_line` and empty glosses. English resolves exactly to
`Copy this line exactly:`; Ukrainian resolves exactly to
`Скопіюйте цей рядок без змін:`. The Ukrainian source requires future human
linguistic/product review before product admission.

Guidance may describe locale, pack version, instruction parts, and optional
glosses. It may not own or mutate stimulus or expected answers, select answer
rules, validate attempts, admit catalog content, create engine truth, perform
client/TUI lookup, localize interface text, translate at runtime, use a network,
or choose a fallback. No product caller exists.

## Pure Guidance Resolution

`src/curriculum/guidance/resolution.rs` reads only an `ExpandedTask` generated
ID, instruction key, and selected assignment slot/rendered/meaning-key facts.
It returns `ResolvedGuidance`, containing exact instruction plus structured
`GuidanceProvenance` (`core_generated_id`, locale, pack version, instruction
key). It does not read or return task stimulus, expected response, answer rule,
validator state, learner result, or interface locale.

Resolution linearly selects one exact case-sensitive locale/version pack and
one exact instruction key. Duplicate packs, duplicate instructions, duplicate
assignments, missing slots or meaning keys, and missing or duplicate glosses are
explicit `GuidanceError` values. `Literal`, `SlotRendered`, and `SlotGloss`
append exact authored or selected bytes without trimming, newline insertion,
normalization, translation, tool use, or fallback. Repeated valid references are
allowed.

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

No implementation lane is active. Language-Agnostic Capability Selector
Architecture V1 is the next planned design-only boundary; materialization
remains paused. Compatibility rows for non-Cartesian systems, product
presentation orchestration, explicit user-selected fallback policy, linguistic
admission, and all catalog admission semantics remain deferred. Update on any
task/guidance source, expansion, resolution, versioning, determinism, identity,
isolation, or admission change.
