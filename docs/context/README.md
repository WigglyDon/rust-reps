# Project Context System

Context role: Canonical law for durable repository context.
Scope: Repository documentation, scoped agent law, context membership, and conflict handling.
Canonical for: Context roles, inheritance, evidence labels, update rules, and Context-SHA.
Not canonical for: Product runtime behavior, terminal output, or Git branch tips.
Inherits: [`../../AGENTS.md`](../../AGENTS.md).
Current-state owner: [`CURRENT_STATE.md`](CURRENT_STATE.md).
Related evidence: [`../evidence/README.md`](../evidence/README.md).
Update triggers: A context owner, evidence class, discovery rule, or manifest algorithm changes.

## One Fact, One Owner

Mutable project truth has one canonical owner. Other pages link to it and add
scope-specific consequences; they do not copy the mutable ledger. Context is
split by role:

- law defines rules that should remain stable;
- current state records the presently verified snapshot;
- history records direction changes without pretending old direction is current;
- decisions record why consequential choices were made and how they reverse;
- contracts define stable interfaces and exact behavior;
- evidence records what a command or observation actually proved.

`LIVE_REPO_FACT` and `RUNTIME_FACT` outrank stale prose in their own scope.
README is descriptive evidence only. Git owns document revision history, so a
context document never embeds its own commit SHA.

## Evidence Language

- `USER_DECISION`: Don's explicit product or process judgment.
- `LIVE_REPO_FACT`: directly observed repository, Git, branch, worktree, file, or source state.
- `RUNTIME_FACT`: directly measured execution behavior with identified conditions and output.
- `WORKER_CLAIM`: an earlier packet or agent report not revalidated here.
- `INFERENCE`: a conclusion derived from identified evidence.
- `UNKNOWN`: unavailable, unverified, ambiguous, contradictory, or unrecoverable.

Confidence and consensus are not evidence. Green tests prove only their tested
contracts. Contradictory sources are reported, not silently blended.

## Recursive Inheritance

Agents read root `AGENTS.md`, then each scoped `AGENTS.md` on the path to the
working directory. A child may narrow authority or add validation, but may not
weaken root law. Canonical pages use links instead of copied mutable truth.

Master owns integration and shared-context correction. A supervisor seed owns
lane meaning, not product-wide truth. Worker Codex discovers live state and
returns evidence. A future verifier checks structure and deterministic hashes;
it cannot decide every semantic contradiction.

## Canonical Context Header

Canonical context documents carry these fields near the top:

```text
Context role:
Scope:
Canonical for:
Not canonical for:
Inherits:
Current-state owner:
Related evidence:
Update triggers:
```

## Context-SHA

[`../../ops/fleet/context-manifest.tsv`](../../ops/fleet/context-manifest.tsv)
is the machine-readable membership owner. Each bytewise-path-sorted line is:

```text
<lowercase-sha256><TAB><repository-relative-path>
```

The manifest excludes itself, generated human views, reported-hash files,
transient packets, proof logs, artifact archives, build output, and unaccepted
candidate reports. Regeneration hashes every listed canonical node first, then
writes the lines in bytewise repository-relative path order.

`Context-SHA` is the lowercase SHA-256 of the exact manifest bytes. The
generated [`../../ops/fleet/context-sha256.txt`](../../ops/fleet/context-sha256.txt)
reports it without creating a hash cycle. `scripts/fleet.sh context-manifest`
regenerates both files; `context-verify` rejects missing, added, changed, or
stale membership and a mismatched reported hash.

## Updates And Conflict Handling

Update the canonical owner when its fact changes, regenerate the manifest, and
send a complete context-delta packet to active lanes. A clean lane may
fast-forward when lineage permits; local or dirty work must be preserved while
context is merged or rebased according to verified convention. Mechanical
context-only conflicts may be resolved without restarting the goal. Semantic
conflicts are reported. `Restart-Goal` defaults to `false`.

Superseded documents remain history or donor/reference material with their old
direction labeled. Filesystem time alone never chooses a winner.
