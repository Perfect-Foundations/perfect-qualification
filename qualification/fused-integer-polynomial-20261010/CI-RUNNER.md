# Hosted GitHub Actions pre-run failure (2026-10-10)

Arithmetic PR #15 latest source SHA: `73c3e6f30278556cda090b8d5cce2ed2d2a422c1`.

Latest directly retrieved GitHub Actions run: `38035599629` — `completed/failure`.
API: `GET /repos/Perfect-Foundations/perfect-arithmetic/actions/runs/38035599629/jobs`.
Nine job records were returned: Fuzz testing; mutation testing; Quality/MSRV; Class A Ubuntu 24.04, Ubuntu 24.04 ARM and Windows 2025; Class B wasm32v1-none, thumbv7em-none-eabihf and riscv64imac-unknown-none-elf. All were marked `failure` with **zero steps** and an empty `runner_name`, with recorded timestamps around `2026-10-10T07:46:53Z–07:46:58Z`. The same outcome occurred for prior runs `38033113694` and `38034325983`. The workflow YAML uses hosted runner labels, a checked-out source commit, a pinned Numeric source-snapshot action and explicit feature/test steps; **none of those steps executed in the observed run**.

This evidence distinguishes a **before-runner** failure from a Rust compiler/test failure. It does **not** establish the ultimate root cause (runner provisioning, repository/organization account limits/settings, platform failure, or GitHub service restriction). No evidence justifies silently changing workflow YAML or purchasing runner minutes. The existing user's instruction is to use free local/VM-equivalent testing where valid, while preserving the GitHub-hosted CI gate as a blocker.

Separately verified: Windows 11 and Ubuntu WSL2 Rust 1.99.0 source/Qualification feature matrices pass. WSL2 is Linux x86-64, **not native ARM64**. Missing CI acceptance remains an explicit unclosed requirement and does not change authoritative `project-status.toml` readiness.
