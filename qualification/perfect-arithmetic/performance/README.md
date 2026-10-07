# Perfect Arithmetic Q6 performance driver

This qualification-owned executable measures the exact Perfect Arithmetic
candidate against direct `num-bigint 0.5.1` in the same process.

The workload shape, deterministic operands, calibration, median timing, and
allocation counting are retained by Perfect Qualification so a candidate cannot
change the measurement generator that decides its own Q6 performance result.
The candidate dependency is rewritten to an exact local path by the Q6 reusable
workflow. The opt-in `candidate-high-throughput` feature maps only to the
candidate's public `high-throughput` feature.
