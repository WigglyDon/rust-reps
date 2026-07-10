# Rust Reps Packet Protocol

Context role: Canonical packet and context-delta protocol.
Scope: Consequential work packets, replies, repairs, lane handoffs, and integration results.
Canonical for: Packet V1 envelope, causality, stale handling, autonomy cadence, and fragment rejection.
Not canonical for: Product behavior or private chat state.
Inherits: [`../../AGENTS.md`](../../AGENTS.md) and [`../context/README.md`](../context/README.md).
Current-state owner: [`../context/CURRENT_STATE.md`](../context/CURRENT_STATE.md).
Related evidence: Packet verifier fixtures and immutable external outbox files.
Update triggers: Envelope, packet type, causality, context-delta, or authority law changes.

## Packet Protocol V1

Every consequential packet contains literal values for:

- `Packet-Version`
- `Packet-ID`
- `Reply-To`
- `From`
- `To`
- `Project-ID`
- `Lane-ID`
- `Goal-ID`
- `Context-SHA`
- `Expected-Base`
- `Worktree`
- `Branch`
- `Authority-State`
- `Expected-Reply-Type`
- `Stale-When`
- `Allowed-Final-Classes`
- `Allowed-Stop-Classes`

Canonical envelope:

```text
BEGIN RUST REPS PROJECT PACKET

Packet-Version: 1
Packet-ID: literal immutable ID
Reply-To: literal packet ID or NONE
From: literal sender
To: literal destination
Project-ID: rust-reps
Lane-ID: literal lane ID
Goal-ID: literal goal ID
Context-SHA: literal lowercase SHA-256 or explicit UNKNOWN class
Expected-Base: literal commit SHA or explicit UNKNOWN class
Worktree: literal absolute path
Branch: literal branch
Authority-State: literal authority description
Expected-Reply-Type: literal reply type
Stale-When: literal stale conditions
Allowed-Final-Classes: literal classes
Allowed-Stop-Classes: literal classes

[complete semantic body]

END RUST REPS PROJECT PACKET
```

The angle-bracket-free phrases above are grammar metavariables in a protocol
example, not operational values. Operational packets replace every phrase with
a literal or an explicit `UNKNOWN_*` class.

## Packet Types

1. Master GPT to Master Codex work packet.
2. Master Codex final report.
3. Supervisor seed.
4. Supervisor to Worker Codex work packet.
5. Worker final report.
6. Focused repair packet.
7. Final lane packet.
8. Red-line decision request.
9. Context-delta amendment.
10. Respawn seed.
11. Integration result.

Each type uses the same envelope and adds body fields appropriate to its
authority. Reports include final class, files changed, validation, divergence,
overlap, artifacts/checksums, a `TEACHING` section, and explicit no-push or
no-deploy statements when authority excludes them.

## Causality And Staleness

- A reply's `Reply-To` names exactly the packet answered, or `NONE` only for a root seed.
- Packet IDs are immutable and unique within the project transport record.
- Packet fragments, missing END markers, and separately interpreted semantic pieces are invalid.
- Stale packets are not silently executed; the recipient reports the literal stale condition.
- `Context-SHA` identifies governing repository memory; `Expected-Base` identifies intended source lineage.
- Unknown facts use explicit `UNKNOWN_*` values. Confidence never fills a field.
- Don transports complete packets and does not fill routine Git, path, auth, or branch facts.
- A packet claim remains a claim until live source/Git/runtime evidence revalidates it.

## Normal Autonomy Cadence

When implementation is authorized, the worker performs discovery and the
bounded implementation in one comprehensive pass. It does not stop at a
discovery-only checkpoint, send routine progress packets, or route ordinary
branch/auth/debug mechanics through Don. Normal return points are merge-ready,
partial-salvage, red-line, or respawn results. One focused repair is normal; a
second requires material progress and a bounded remainder. Workers never push
unless a packet explicitly grants that authority.

## Literal Rust Reps Work Example

This is a complete protocol example whose `UNKNOWN_*` value is an explicit
evidence class, not an unresolved placeholder:

```text
BEGIN RUST REPS PROJECT PACKET

Packet-Version: 1
Packet-ID: RR-EXAMPLE-STATIC-AUDIT-001
Reply-To: NONE
From: Master GPT
To: Worker Codex
Project-ID: rust-reps
Lane-ID: example-static-audit
Goal-ID: verify-static-boundary
Context-SHA: UNKNOWN_CONTEXT_NOT_YET_ACCEPTED
Expected-Base: 5f8a1d9504234f909760d55e026c46638aad602a
Worktree: /home/don/rust-reps-example-audit
Branch: audit/example-static-boundary
Authority-State: read-only inspection; no edits, commits, pushes, or deploys
Expected-Reply-Type: WORKER_FINAL_REPORT_V1
Stale-When: expected base differs or unknown dirty work exists
Allowed-Final-Classes: ACCEPTED, PARTIAL, STOP
Allowed-Stop-Classes: UNKNOWN_WORK_AT_RISK, VALIDATION_FAILURE

Inspect the static boundary and return one complete report.
No push or deploy is authorized.

END RUST REPS PROJECT PACKET
```

## Context-Delta Packet

A context delta is an immutable packet with the normal envelope plus:

- `Old-Context-SHA`
- `New-Context-SHA`
- `Changed-Context-Files`
- `Source-Commit`
- `Branch-Update-Instruction`
- `Root-To-Local-AGENTS-Reread`
- `Docs-INDEX-Reread`
- `Subsystem-Page-Reread`
- `Lane-Page-Reread`
- `Current-State-Reread`
- `Contradiction-Report-Instruction`
- `Restart-Goal`

Default `Restart-Goal: false`. A clean lane fast-forwards when lineage permits.
Local commits merge or rebase according to verified convention. Dirty work is
preserved while context is integrated safely. Context-only conflicts may be
resolved mechanically without changing local semantics; semantic conflicts are
reported. The packet carries exact worktree, branch, expected base, update
method, no-push/no-deploy law, and expected reply type.

## Context-SHA Link

The algorithm and membership owner are defined in
[`../context/README.md`](../context/README.md). Packet generation must first run
`./scripts/fleet.sh context-verify --repo <literal-repo-path> --machine` and use
the reported hash. The text in angle brackets here is explicitly a grammar
metavariable, not an operational packet field.
