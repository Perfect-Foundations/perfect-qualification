# Independent Rational candidate qualification

This isolated test-only Cargo package qualifies Rational PR #7 at Git
857c55beee5aeb381c674d739630b8c3407fa7f8 against the same independent
external-reference test in the five-foundation integration consumer.
Arithmetic is pinned to 95e8499f458d9cb681ce0d452cade1cd17c9f6ad.

Do not add this package as a production dependency or merge its candidate
source into the existing five-foundation integration resolution.
Float and Decimal currently require Rational main at
21a7fba5f22ab7f5db12b480a1c910edd1797d77.

This initially qualified Rational alone. The later downstream candidate
consumer below adds exact-source Windows cross-family evidence, but does not
establish Linux/ARM64 runtime, mutation/fuzz, full performance/reference
qualification, Q0-Q6 completion, or private R7A readiness.

## Exact downstream candidate graph (2026-10-24)

This same test-only consumer now also resolves source-exact Float candidate
`194887d5fa2959ce758160de3354376b1a4baaee`, derived from Float PR #4,
and Decimal candidate `f75df906f3ab83aa5ddb8eb94328c146a96a11f6`,
derived from Decimal main. Both pin Rational candidate
`857c55beee5aeb381c674d739630b8c3407fa7f8`; Numeric and
Arithmetic keep their established exact revisions.

The tests reuse, by source path and without copying fixture implementations,
the existing five-foundation Qualification oracle modules. Cargo's resolved
normal dependency graph must contain exactly one identity for each Perfect
production crate before any PASS is recorded.

Neither production main nor the existing main-source integration graph is
repinned. This is candidate-only qualification, not merge or release approval.

Windows Rust 1.99.0 verification of this exact five-foundation candidate graph:
locked offline no-default and all-features suites 12/12 each, plus
warning-denied Clippy and rustfmt PASS. Cargo's normal-edge dependency graph
shows no duplicate Perfect crate identities. These results are limited to
the tested Windows configuration and the exact candidate Git SHAs above.


## Mutation-derived exact revision update

Earlier candidate source `0f8ccd07b1f3c29b1d725ef79fb6dfa48274f9b2`
allowed two extractor mutations to survive its primitive-only test.
Rational PR #7 now includes a deterministic certified-progress regression
at `857c55beee5aeb381c674d739630b8c3407fa7f8`.
Corresponding downstream Float/Decimal pins were updated and retested.
The old results are preserved as historical evidence only, not transferred
automatically to the newer source graph.
