# Perfect Arithmetic Q6 fuzz targets

These fuzz targets are owned by Perfect Qualification rather than the candidate
under test. The reusable Q6 workflow rewrites only the exact
`perfect-arithmetic` dependency to the supplied candidate checkout.

The targets exercise arithmetic reconstruction, canonical sign/magnitude
behavior, shifts/GCD, and deterministic public Hash observations. Candidate
changes cannot replace these target bodies.
