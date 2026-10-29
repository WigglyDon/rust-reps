# Verification and evidence operations

After resource admission, run `cargo xtask verify` from the checkout for a source
or runtime delivery. The gate checks workspace formatting,
warnings-denied Clippy, workspace tests, current context, actual CLI transcripts,
temporary installation and working/staged diff checks. The temporary installation
does not replace the user's installed command. `scripts/proof.sh` remains a
standalone product-proof entry; an aggregate verification avoids rerunning
unchanged formatting/tests inside that proof.

Cargo-based entry points may compile their helper. For a bounded documentation
review, an already-built developer helper whose source/lockfile identity has
been checked can run `context current --verify`, `context manifest` and
`context verify` directly, without Cargo. If that helper is absent, report the
prerequisite and use read-only Git/hash observations before planning a build.
Documentation-only changes need applicable context/link/hash and diff checks;
do not describe those as a rerun of the full compilation/transcript gate.

Verification and the proof script can grow compiler caches, temporary files and
the private install root. Use admitted owned target/TMPDIR destinations, current
capacity and serialization as described in [development](../DEVELOPMENT.md).
Do not reuse a retired destination or overwrite a user's installed command to
make a check pass. Preserve required evidence independently of disposable caches.

`cargo xtask context current --verify` reports local source/context identity.
Its origin-main SHA and divergence use cached refs. Fetch separately for fresh
remote evidence. Credentials and raw helper configuration are not report data.

Context membership is [declared here](../../ops/fleet/context-manifest.tsv).
After reviewing owner changes, use `cargo xtask context manifest`, then
`cargo xtask context verify`. Generation is explicit; verification is read-only.
Raw logs/private evidence belong in ignored local output, not Git or public issues.

Historical queue, lane, packet, respawn and evidence-manifest commands remain
available through `cargo xtask` or `scripts/fleet.sh`. They diagnose supplied
records and never integrate, publish, launch a worker or reactivate a lane.
The old [queue](../../ops/fleet/integration-queue.tsv) is dormant; its generated
[view](../lanes/integration-queue.md) is historical infrastructure.

Tests of code consistency, real-terminal observations, learner feedback and
physical-hardware measurements have different scope. Report exact conditions,
failures and limits. A failed check blocks its affected delivery, not unrelated
safe work. [Development](../DEVELOPMENT.md) owns publication authority and
[the evidence guide](../evidence/README.md) owns claim discipline.
