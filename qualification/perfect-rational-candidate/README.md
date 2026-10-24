# Independent Rational candidate qualification

This isolated test-only Cargo package qualifies Rational PR #7 at Git
0f8ccd07b1f3c29b1d725ef79fb6dfa48274f9b2 against the same independent
external-reference test in the five-foundation integration consumer.
Arithmetic is pinned to 95e8499f458d9cb681ce0d452cade1cd17c9f6ad.

Do not add this package as a production dependency or merge its candidate
source into the existing five-foundation integration resolution.
Float and Decimal currently require Rational main at
21a7fba5f22ab7f5db12b480a1c910edd1797d77.

This is source-exact candidate Q5 evidence only. It does not establish
Linux or ARM64 runtime, mutation/fuzz, downstream candidate compatibility,
Q0-Q6 completion, or private R7A readiness.