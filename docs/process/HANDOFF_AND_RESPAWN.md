# Handoff And Respawn

Context role: Canonical stale-session and replacement-seed protocol.
Scope: Supervisors, Worker Codex sessions, donor preservation, and fresh lane continuation.
Canonical for: Staleness, respawn triggers, bundle contents, and deactivation rules.
Not canonical for: Product behavior or automatic branch mutation.
Inherits: [`../../AGENTS.md`](../../AGENTS.md) and [`PACKET_PROTOCOL.md`](PACKET_PROTOCOL.md).
Current-state owner: [`../context/CURRENT_STATE.md`](../context/CURRENT_STATE.md).
Related evidence: Lane registry, respawn-bundle output, and immutable packet archive.
Update triggers: Stale criteria, bundle schema, preservation law, or authority changes.

## Stale Conditions

A supervisor or worker is stale when its Context-SHA or accepted base is
superseded, its lane page no longer matches live worktree identity, its authority
was revoked, unresolved contradictions invalidate the goal, or it cannot return
a complete packet. Silence or age alone is not proof of staleness.

Respawn is appropriate when a stale session cannot safely absorb a context
delta, private state cannot be recovered, worktree lineage is wrong, or a fresh
bounded worker is clearer than repeated repairs. A respawn must increase clarity
and preserve evidence.

## Preservation And Replacement

- Never delete useful old work, reset, clean, stash unknown changes, or repurpose an unrelated branch.
- Retain stale worktrees/branches as donor/reference until Master classifies them.
- Create a fresh worktree from accepted main only after live verification.
- Deactivate the stale session in lane memory; do not imply its branch was deleted.
- Manual packet transport remains complete and immutable.

## Complete Replacement Seed

A respawn seed includes project identity, current Context-SHA, current goal,
accepted base, latest candidate from the declared Worker Codex worktree,
worktree, branch, clean/dirty state, artifact references, unresolved risks,
exact next action, authority state, prohibited authority, expected proof,
stale-session identity, deactivation instruction, `Restart-Goal: false`,
no-push/no-deploy law, and donor/unknown-work preservation law. It never infers
private chat state. Missing required lane/base data is an explicit error.

The lane page declares the supervisor destination and expected reply type. The
generated packet uses those literal values, replies to the lane's latest packet
ID, and derives its state-specific packet ID from the lane, Context-SHA, and
candidate HEAD. Duplicate lane fields are conflicts and fail generation.

Before writing output, the generator verifies the complete packet through the
packet verifier. It reads the candidate HEAD, branch, and status from the exact
declared Worker Codex worktree. A missing worktree or branch mismatch fails. A
dirty worker remains eligible for handoff, but the packet must report that state
and require preservation and inspection before editing.

The deterministic generator is:

```text
./scripts/fleet.sh respawn-bundle --repo /literal/repository/path --lane literal-lane-id --output /literal/output/path
```

Paths above are protocol examples. Operational invocations use actual literal
paths and are captured as evidence.
