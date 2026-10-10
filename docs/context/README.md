# Context ownership

Keep each durable fact at one owner. Root/scoped instructions own stable law;
[product direction](../PRODUCT.md) owns purpose; [curriculum design](../CURRICULUM.md)
owns teaching direction; [current state](CURRENT_STATE.md) owns implemented and
unresolved status; [decisions](DECISION_LOG.md) own accepted choices; source and
runtime establish actual behavior. [The index](../INDEX.md) routes to those
owners rather than copying their status.

Distinguish Don's decisions, observed repository/runtime facts, outside evidence,
agent claims, inference and unknowns whenever the distinction matters. A passing
test proves its exercised contract, not learning quality or physical accuracy.
Confidence, repetition and agreement do not upgrade evidence. Report a conflict
and preserve unknown work rather than silently reconciling consequential meaning.

Read the effective root-to-local instructions and the actual task. Historical
lane seeds, old packets and chat summaries are references, never activation
instructions. Git owns document revision history; canonical pages do not embed
their own revision hash or duplicate task status.

## Context manifest

[The manifest](../../ops/fleet/context-manifest.tsv) owns checked membership.
Its bytewise-path-sorted rows are lowercase SHA-256, tab, repository-relative
path. The manifest, reported hash and build/private outputs are excluded from
membership. [The reported hash](../../ops/fleet/context-sha256.txt) is SHA-256
of the exact manifest bytes.

`cargo xtask context manifest` regenerates declared membership and hashes after
review. `cargo xtask context verify` checks required members, paths, hashes,
relative links and the reported digest without mutation. A digest proves file
identity, not semantic agreement. No mandatory prose headers, canned status
phrases or label inventory are imposed by the verifier.

New owners must be added explicitly to membership. Preserve historical material
with its historical role visible. Update the actual owner when an accepted
fact changes and discuss material shared contract changes on the issue.
