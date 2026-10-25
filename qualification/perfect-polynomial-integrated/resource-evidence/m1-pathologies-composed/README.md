# Bounded M1 pathological resource census — source exact combined PR #14

**Production source:** `34ffc473bf07960b66fdc8b0637bb0bf4aa8670e` (composed metadata and test-only amendments to PR #8, production mathematical code unchanged). **Rust 1.99.0**, `cargo --release --offline --locked` on Windows 11 x86-64 and Ubuntu WSL2 Linux x86-64.

`qualification/perfect-polynomial-integrated/examples/resource_pathologies.rs` produces deterministic operand shapes and performs algebraic correctness assertions outside the timing/accounting window: `product(1)=a(1)b(1)`, expected output degrees, and existing known exact GCD reference vectors for PRS cases. It executes one warm-up then one **complete public operation plus result drop** under a test-only `GlobalAlloc` counter, and repeats in three separate host process executions per case. No production unsafe code is added. New matrix covers **23 distinct cases × 2 hosts × 3 processes** = 138 measurements. Raw `windows-*.csv`, `linux-*.csv`, `summary.csv` and Linux GNU-time process-level reports are retained here.

Scope: output `elapsed_ns` is one-shot exploratory latency, not a calibrated statistical benchmark; mean/min/max across three processes are in summary.csv. `allocs` and `reallocs` measure allocator call counts, `total_requested_bytes` measures allocation/reallocation request sizes, `peak_incremental_live_bytes` measures requested bytes held within one operation and excludes persistent input operands and allocator slack, and `net_live_delta=0` is expected after result drop. No claim is made of peak physical heap, stack, memory safety proof, universal latency or successful OOM recovery. The GNU time reported maximum RSS is **whole executable high-water**, not per-case. Corrected uniform input fixture asserts that *both* operands' Rational coefficient denominators are uniform; this is a different dispatch from bounded distinct-denominator clearing.

### Representative stable allocation census

| Case | Allocations | Requested bytes | Peak incremental live bytes |
|---|---:|---:|---:|
| Dense Z[x] degree 256×256, 32-bit coeffs | 61,506 | 1,000,496 | 24,336 |
| Sparse Z[x] degree 512×512, 32-bit coeffs | 1 | 32,800 | 32,800 |
| Rational uniform denominators, 256 bits | 31 | 2,008 | 1,256 |
| Rational differing denominators, 512-bit LCM | 64 | 5,672 | 2,000 |
| Rational differing denominators, 513-bit LCM (exact fallback) | 55 | 4,520 | 1,088 |
| Z[x] late PRS continuation | 892 | 130,936 | 9,208 |
| Z[x] early PRS 4,096-bit guard refusal | 118 | 67,088 | 14,768 |

Trends: dense schoolbook work dominates as polynomial degree grows; sparse inputs use little arithmetic but still require degree-sum dense result capacity (1,025 Integer slots and 32,800 requested bytes for degree-512 operands). At this small exact Rational fixture the 513-bit fallback is slightly faster and cheaper than accepted 512-bit bounded LCM; the 512-bit value remains a **resource bound**, not an empirically proven universal speed crossover. PRS internal guard limits do not bound the public Euclidean fallback, which retains mathematical exactness but may consume substantial memory for adversarial untrusted inputs. The production infallible multiplication API has no configurable memory budget or typed OOM guarantee, already documented in `src/lib.rs`.

Native physical ARM64, peak stack, allocator slack/RSS per-case, exhaustive degrees and independent authorized M1 review remain unavailable/not accepted. Do not infer full M1/Q6 gate completion.
