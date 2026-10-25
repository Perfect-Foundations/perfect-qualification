#!/usr/bin/env bash
set -euo pipefail
export PATH="$HOME/.cargo/bin:$PATH" CARGO_NET_OFFLINE=true
work="$(mktemp -d /home/ubuntu/pf-verify/interval-arb-m2-20261009.XXXXXXXX)"
summary="$work/summary.txt"
echo "WORKSPACE=$work" | tee "$summary"
clone_exact(){
  name="$1";source="$2";sha="$3"
  git clone --quiet --no-hardlinks "$source" "$work/$name"
  git -C "$work/$name" -c advice.detachedHead=false checkout --quiet --detach "$sha"
  test "$(git -C "$work/$name" rev-parse HEAD)" = "$sha"
  echo "VERIFIED_SOURCE_$name=$sha" | tee -a "$summary"
}
clone_exact qualification /mnt/c/Dev/PerfectFoundations/_qualification_interval_m1_20261008 215ec5f3f5f72ad0e200a0dee3034f97fa5b68ea
base=/home/ubuntu/pf-verify/interval-m1-linux-exact-20261008
clone_exact interval "$base/interval" 30aca00817bb79f0be3bf43e96c767db9fe45bc5
clone_exact float "$base/float" bbd8b9e4aac0015facf0bb75819049cee68dc0e7
clone_exact arithmetic "$base/arithmetic" 1a54d3c7cbbae4e73325cc70fd2777a2427b1504
clone_exact rational "$base/rational" 35a8e9cc629ee578fe7b624e2134929ce7eeff8a
clone_exact numeric "$base/numeric" 19b6747cd852a47a694a020b97ba70b6b3ef259b
export PFQ_EXACT_SOURCE_ROOT="$work" CARGO_TARGET_DIR="$work/target"
suite="$work/qualification/qualification/perfect-interval"
manifest="$suite/Cargo.toml"
python3 "$suite/scripts/materialize_exact.py" > "$work/exact-graph.log"
want=8bb8011b5bf4e3292ea631df2d8e5f13bbdbb6e08f1df2eea8402b39fd9f9e57
test "$(sha256sum "$suite/vectors/arb_adversarial_v1.tsv" | cut -c1-64)" = "$want"
python3 "$suite/scripts/generate_arb_adversarial.py" > "$work/mpfr-generation.log"
test "$(sha256sum "$suite/vectors/arb_adversarial_v1.tsv" | cut -c1-64)" = "$want"
python3 "$suite/scripts/verify_arb_output.py" > "$work/arb-stored-certified.log"
flint=/home/ubuntu/pf-verify/interval-flint-arb-20261009/sysroot
inc="$flint/usr/include";lib="$flint/usr/lib/x86_64-linux-gnu"
cc -std=c11 -O2 -Wall -Wextra -Werror -I"$inc" -I"$inc/x86_64-linux-gnu" -L"$lib" -Wl,-rpath,"$lib" "$suite/scripts/flint_arb_oracle.c" -lflint -o "$work/flint-arb-oracle"
"$work/flint-arb-oracle" "$suite/vectors/arb_adversarial_v1.tsv" > "$work/arb-live.tsv" 2> "$work/flint-run.log"
python3 "$suite/scripts/verify_arb_output.py" "$work/arb-live.tsv" | tee "$work/arb-live-certified.log"
echo "LIVE_FLINT_ARB_292_FINITE_PASS_38_UNSUPPORTED" | tee -a "$summary"
rustc +1.99.0 --version | tee -a "$summary"
echo "HOST=$(uname -sm)" | tee -a "$summary"
check(){
 label="$1";shift
 if timeout -k 10s 180s "$@" > "$work/$label.stdout" 2> "$work/$label.stderr";then
   echo "PASS=$label EXIT=0" | tee -a "$summary"
   grep -E 'test result:|ADVERSARIAL_.*=' "$work/$label.stdout" | tee -a "$summary" || true
 else
   code=$?
   echo "FAIL=$label EXIT=$code" | tee -a "$summary"
   tail -40 "$work/$label.stderr";exit "$code"
 fi
}
check lock cargo +1.99.0 generate-lockfile --offline --manifest-path "$manifest"
check fmt cargo +1.99.0 fmt --manifest-path "$manifest" --all -- --check
for features in all-features no-default-features;do
  for target in arb_adversarial domain_semantics;do
    check "$target-$features" cargo +1.99.0 test --offline --locked --manifest-path "$manifest" --"$features" --test "$target" -- --nocapture
  done
done
check clippy-all cargo +1.99.0 clippy --offline --locked --manifest-path "$manifest" --all-targets --all-features -- -D warnings
check clippy-nodefault cargo +1.99.0 clippy --offline --locked --manifest-path "$manifest" --all-targets --no-default-features -- -D warnings
sha256sum "$suite/Cargo.lock" | tee -a "$summary"
echo "FINAL_NEW_ARB_ADVERSARIAL_LINUX_NATIVE_X86_64_PASS" | tee -a "$summary"
