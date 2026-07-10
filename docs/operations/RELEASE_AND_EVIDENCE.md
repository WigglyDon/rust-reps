# Release And Evidence Operations

Context role: Canonical integration/evidence operations law.
Scope: Context generation, queue rendering, verification, artifacts, and deferred release wiring.
Canonical for: Machine owners, generation commands, failure/rollback behavior, and authority limits.
Not canonical for: Product release approval or current branch tips.
Inherits: [`../../AGENTS.md`](../../AGENTS.md) and [`../context/README.md`](../context/README.md).
Current-state owner: [`../context/CURRENT_STATE.md`](../context/CURRENT_STATE.md).
Related evidence: [`../evidence/RUNTIME_EVIDENCE_SCHEMAS.md`](../evidence/RUNTIME_EVIDENCE_SCHEMAS.md).
Update triggers: Manifest, queue, evidence generator, release gate, or authority changes.

## Canonical Owners And Generated Views

- Context membership owner: [`../../ops/fleet/context-manifest.tsv`](../../ops/fleet/context-manifest.tsv).
- Generated reported Context-SHA: [`../../ops/fleet/context-sha256.txt`](../../ops/fleet/context-sha256.txt).
- Integration queue owner: [`../../ops/fleet/integration-queue.tsv`](../../ops/fleet/integration-queue.tsv).
- Generated queue view: [`../lanes/integration-queue.md`](../lanes/integration-queue.md).
- Evidence schema: [`../evidence/RUNTIME_EVIDENCE_SCHEMAS.md`](../evidence/RUNTIME_EVIDENCE_SCHEMAS.md).
- Final evidence manifests: external checksummed artifact directory only.

## Commands

```text
./scripts/fleet.sh context-manifest --repo /literal/repository/path
./scripts/fleet.sh context-verify --repo /literal/repository/path --human
./scripts/fleet.sh queue-render --repo /literal/repository/path
./scripts/fleet.sh queue-verify --repo /literal/repository/path --machine
./scripts/fleet.sh evidence-manifest --input /literal/input.tsv --output /literal/output.tsv
```

Example paths above are grammar examples; evidence records actual literal paths.
Generation is deterministic and fails on malformed input, missing members,
stale hashes/views, duplicate lane states, or missing evidence fields.

## Queue Schema And State

`integration-queue.tsv` has one header followed by zero or more rows with:
lane ID, candidate HEAD, base, Context-SHA, artifact, targeted validation,
full-verify status, overlap risk, dependency, authority impact, revalidation
state, integration decision, and push state. Tabs are delimiters; embedded tabs
or newlines are forbidden. The Markdown view is never hand-maintained.

The current queue may be empty while a lane awaits its own immutable final
report; this avoids self-referential commit SHAs. The external report carries
the exact final candidate SHA for Master review.

## Failure, Rollback, And Wiring

Stale/malformed data fails nonzero and leaves canonical source unchanged unless
the explicit generator command was requested. Rollback restores the prior TSV
or context commit and regenerates derived views. Diagnostics never integrate,
push, deploy, or retire lanes.

Central `scripts/proof.sh` wiring is deferred. Fleet checks remain independently
runnable until Master accepts ownership and confirms no active-lane overlap.
Existing product transcript proof remains unchanged.
