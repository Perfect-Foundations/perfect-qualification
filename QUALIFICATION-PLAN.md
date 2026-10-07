# Qualification Plan

## Gates

### Q0 — repository integrity
- expected repository and revision
- clean source state
- dependency inventory
- declared MSRV/targets/features
- required historical-reuse disposition and exact predecessor revisions present

### Q1 — build matrix
- stable/MSRV
- supported operating systems
- supported architectures
- `no_std` where claimed
- feature combinations

### Q2 — semantic integration
- cross-crate conversions
- exactness/rounding contracts
- canonical wire compatibility
- error propagation
- units/uncertainty compatibility where applicable

### Q3 — reproducibility
- deterministic outputs
- seeded stochastic behavior
- order-invariance where promised
- canonical-byte stability

### Q4 — robustness
- fuzzing
- mutation testing
- sanitizer/Miri coverage where applicable
- malformed-input handling
- applicable LMES/Perfectπ historical defect classes exercised or technically NOT_APPLICABLE

### Q5 — external reference
- known-answer vectors
- independent implementations
- standards/reference datasets
- numerical error analysis
- adapted predecessor implementation is not used as its own independent oracle

### Q6 — release candidate
- full supported matrix
- documentation consistency
- dependency/FFI review
- performance regression review
- M1/M3/M5/M6 historical-reuse catch-up complete; unresolved release-blocking deferrals absent
- evidence bundle

Qualification requirements are crate-specific; not every test class applies to every domain.
