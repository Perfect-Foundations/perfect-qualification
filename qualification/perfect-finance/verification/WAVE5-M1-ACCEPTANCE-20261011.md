# Wave 5 source-exact Qualification evidence

Finance source: `f716cb02c34dc79d51cef3540d7f0c0cc2ba930f` (exact reducer in 705b92b; workflow-only follow-up). Error Correction source: `6c9023a2ecd16f7e6f0b0d9c68df57b38fc497f7` (MDS/radius API in b77c7c6; workflow-only follow-up). Both draft and unmerged.

Finance mathematical references: 188 rows (36 day-count, 48 quantization, 16 prior cashflow, 88 new multioperand). New 88 comprise 48 representable totals and 40 refusals; 528 order variants per suite execution. New frozen SHA256: `c59cf1b20dc18370ba0103f7ea372f17ef7121acfc839d3ace42875ac9e6b455`. FEC mathematical reference rows remain 45, with 2,250 independent GF5 injected decoder patterns and a new public MDS capacity formula test. No repeated execution is counted as a new mathematical reference.

Consumer Git-canonical LF-normalized Cargo.lock SHA256: Finance `9f07a9cecaa2708a5bd6d605c4fecb3bd2e29b92c2d587d3b4ca597f78a4346b`; FEC `36c82dc3208815d7b2b15f400017eaf1a1fc57019771ad94d01621d7fe14755d`. Windows native x86-64 and WSL2 Linux x86-64 tested locked/offline both feature modes, warning-denied Clippy, independent Python oracles, SHA/graph verification. Same physical CPU. Native source tests, rustdoc and configured three no_std compile-only targets passed; no hardware ARM64 or hosted workflow result.

The optimized dual-revision Finance benchmark harness and original measurement evidence are retained alongside this file. Hosted workflows skip draft PRs to conserve GitHub minutes. No formal M1/Q6 acceptance or release.
