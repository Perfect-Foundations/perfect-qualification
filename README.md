<div align="center">

<img src="docs/assets/brand/logo.svg" width="128" alt="Perfect qualification logo">

# Perfect Qualification

### Cross-family verification, compatibility, determinism, and release assurance for Perfect Foundations.

![Role](https://img.shields.io/badge/role-integration%20assurance-0891b2)
![Family](https://img.shields.io/badge/family-Perfect%20Foundations-44546a)
![Production dependency](https://img.shields.io/badge/production%20dependency-never-8b0000)

</div>

---

## Why this repository exists

A crate can pass every one of its own tests and still fail when composed with neighboring crates.

Perfect Qualification exists to answer the harder question:

> **Do supported combinations of Perfect-family crates continue to satisfy the shared family contract together?**

This repository therefore owns **integration evidence**, not production algorithms.

No production Perfect crate should depend on `perfect-qualification`.

---

## Qualification model

| Inputs to qualification |  | Integration |  | Evidence |  | Outcome |
|---|:---:|---|:---:|---|:---:|---|
| Crate-specific tests · Independent references · Target/feature matrices · Determinism checks · Fuzzing/mutation/sanitizers | → | **Supported crate combinations** | → | **Qualification evidence** | → | **Release confidence** |

---

## What gets qualified

### 🔗 Compatibility

- Cross-crate API and version compatibility.
- Dependency-DAG and layering rules.
- Optional-feature combinations.
- Canonical representations across crate boundaries.
- Numeric conversions and precision preservation.
- Units/uncertainty/evidence interoperability where supported.

### 🧱 Build & target coverage

- Declared MSRV.
- Stable toolchain.
- Supported operating systems.
- Supported CPU architectures.
- `no_std` configurations where claimed.
- Feature matrices.
- Optional accelerators/adapters without contaminating portable cores.

### 🎯 Determinism & reproducibility

- Repeated-run equivalence where promised.
- Seeded stochastic reproducibility.
- Canonical-byte stability.
- Order-invariance where explicitly promised.
- Cross-platform consistency within the documented contract.

### 🧪 Robustness

- Integration fuzzing.
- Mutation testing where valuable.
- Sanitizers and Miri where applicable.
- Malformed-input behavior.
- Boundary/degenerate-case behavior.
- Foreign-interface failure isolation.

### 📐 External reference evidence

- Known-answer vectors.
- Independent reference generators.
- Published standards datasets.
- Differential tests against mature external implementations.
- Numerical error analysis.
- Historical dataset regression where standards evolve.

---

## Requirements-to-evidence traceability

Qualification should be traceable back to the contract being qualified.

For applicable release claims, the desired chain is:

**Requirement → ADR / design → Implementation → Verification evidence → Qualification gate**

This means a qualification result should be able to identify:

- the stable requirement ID being demonstrated;
- the architecture decision/design defining the intended behavior;
- the implementation revision/configuration under test;
- the retained test/reference/benchmark evidence;
- the qualification gate and outcome.

A passing test without a known requirement can still discover defects, but it is weaker release evidence than a traceable test tied to the public/semantic contract.

Family rules:

- [Architecture Decision Record Standard](https://github.com/Perfect-Foundations/perfect-family/blob/main/docs/ADR-STANDARD.md)
- [Requirements & Traceability Standard](https://github.com/Perfect-Foundations/perfect-family/blob/main/docs/REQUIREMENTS-TRACEABILITY.md)
- [Glossary & Terminology](https://github.com/Perfect-Foundations/perfect-family/blob/main/docs/GLOSSARY.md)

---

## Qualification gates

| Gate | Name | Purpose |
|---|---|---|
| **Q0** | Repository integrity | Revision, source state, dependency inventory, declared targets/features |
| **Q1** | Build matrix | MSRV, stable, OS/arch, `no_std`, feature combinations |
| **Q2** | Semantic integration | Cross-crate conversions, exactness, rounding, units, uncertainty, canonical bytes |
| **Q3** | Reproducibility | Deterministic outputs, seeded stochastic behavior, byte stability |
| **Q4** | Robustness | Fuzzing, mutation, sanitizers/Miri, malformed inputs |
| **Q5** | External reference | Known-answer vectors, standards, independent implementations |
| **Q6** | Release candidate | Full matrix, docs, FFI/dependency review, performance/evidence bundle |

Not every test class applies to every crate. Qualification is **domain-specific but systematically recorded**.

---

## Evidence principle

A passing crate test suite demonstrates:

> **The crate satisfies the tested parts of its own contract.**

A passing Perfect Qualification run demonstrates:

> **The tested supported combination satisfies the cross-family integration contract.**

Neither statement should be inflated into claims that were not actually tested.

---

## Planned repository structure

```text
perfect-qualification/
├── qualification/   # plans, matrices, gates
├── vectors/         # cross-crate known-answer/reference vectors
├── fixtures/        # integration fixtures and datasets
├── scripts/         # deterministic qualification runners
└── reports/         # retained/generated evidence where appropriate
```

---

## Release philosophy

Qualification is not a ceremony added at the end.

The family should be designed so that:

1. each crate defines what must remain true;
2. those truths can be tested independently;
3. integration boundaries can be exercised deterministically;
4. release evidence can be regenerated;
5. changes that invalidate assumptions become visible early.

---

## Retained qualification reports

- [Perfect Numeric — Q0-Q6 at `19b6747`](reports/perfect-numeric/19b6747-q0-q6.md) — retained/private candidate fully qualified; public-release gates remain explicitly deferred.
- [Perfect Numeric — Q0-Q5 at `ef85b646`](reports/perfect-numeric/ef85b646-q0-q5.md) — independent semantic qualification baseline.
- [Perfect Numeric — superseded Q6 blocker report at `84043a2`](reports/perfect-numeric/84043a2-q6-open.md) — historical evidence from the earlier over-strict Q6 gate definition.

---

## Related

- 🗺️ [Perfect Family architecture](https://github.com/Perfect-Foundations/perfect-family)
- 🏠 [Perfect Foundations organization](https://github.com/Perfect-Foundations)
- 📋 [Perfect Family Project](https://github.com/orgs/Perfect-Foundations/projects/1)
- 📄 [Qualification Plan](QUALIFICATION-PLAN.md)

---

<div align="center">

### A family is only as strong as the contracts between its parts.

</div>
